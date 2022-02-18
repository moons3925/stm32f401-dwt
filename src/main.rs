#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use cortex_m::delay;
use stm32f4::stm32f401;

#[entry]
fn main() -> ! {
    let dp = stm32f401::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // PLLSRC = HSI (default)
    dp.RCC.pllcfgr.modify(|_, w| w.pllp().div4());  // P=4
    dp.RCC.pllcfgr.modify(|_, w| unsafe { w.plln().bits(336) });    // N=336
    // PLLM = 16 (default)

    dp.RCC.cfgr.modify(|_, w| w.ppre1().div2());    // APB1 PSC = 1/2

    dp.RCC.cr.modify(|_, w| w.pllon().on());    // PLL On
    while dp.RCC.cr.read().pllrdy().is_not_ready() {    // 安定するまで待つ
        // PLLがロックするまで待つ (PLLRDY)
    }

    // データシートのテーブル15より
    dp.FLASH.acr.modify(|_,w| w.latency().bits(2));    // レイテンシの設定: 2ウェイト (3.3V, 84MHz)

    dp.RCC.cfgr.modify(|_,w| w.sw().pll()); // sysclk = PLL
    while !dp.RCC.cfgr.read().sws().is_pll() {  // SWS システムクロックソースがPLLになるまで待つ
    }
    // SYSCLK = 16M * 1/M * N * 1/P = 84MHz (= AHB Clock)

    let mut delay = delay::Delay::new(cp.SYST, 84000000_u32);

    cp.DWT.enable_cycle_counter();  // (1)
    let mut msecond: u32 = 1000;
    let mut usecond: u32 = 1000000;

    loop {
        cp.DWT.set_cycle_count(0_u32);  // (2)
        delay.delay_ms(msecond);  // (3)
        usecond = get_usec();  // (4)
        msecond = get_msec();  // (4)
    }
}

fn get_msec() -> u32 {
    stm32f401::DWT::cycle_count() / 84000_u32
}

fn get_usec() -> u32 {
    stm32f401::DWT::cycle_count() / 84_u32
}
