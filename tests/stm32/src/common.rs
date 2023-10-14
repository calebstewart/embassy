#![macro_use]

pub use defmt::*;
#[allow(unused)]
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

#[cfg(feature = "stm32f103c8")]
teleprobe_meta::target!(b"bluepill-stm32f103c8");
#[cfg(feature = "stm32g491re")]
teleprobe_meta::target!(b"nucleo-stm32g491re");
#[cfg(feature = "stm32g071rb")]
teleprobe_meta::target!(b"nucleo-stm32g071rb");
#[cfg(feature = "stm32f429zi")]
teleprobe_meta::target!(b"nucleo-stm32f429zi");
#[cfg(feature = "stm32wb55rg")]
teleprobe_meta::target!(b"nucleo-stm32wb55rg");
#[cfg(feature = "stm32h755zi")]
teleprobe_meta::target!(b"nucleo-stm32h755zi");
#[cfg(feature = "stm32u585ai")]
teleprobe_meta::target!(b"iot-stm32u585ai");
#[cfg(feature = "stm32h563zi")]
teleprobe_meta::target!(b"nucleo-stm32h563zi");
#[cfg(feature = "stm32c031c6")]
teleprobe_meta::target!(b"nucleo-stm32c031c6");
#[cfg(feature = "stm32l073rz")]
teleprobe_meta::target!(b"nucleo-stm32l073rz");
#[cfg(feature = "stm32l152re")]
teleprobe_meta::target!(b"nucleo-stm32l152re");
#[cfg(feature = "stm32l4a6zg")]
teleprobe_meta::target!(b"nucleo-stm32l4a6zg");
#[cfg(feature = "stm32l4r5zi")]
teleprobe_meta::target!(b"nucleo-stm32l4r5zi");
#[cfg(feature = "stm32l552ze")]
teleprobe_meta::target!(b"nucleo-stm32l552ze");
#[cfg(feature = "stm32f767zi")]
teleprobe_meta::target!(b"nucleo-stm32f767zi");
#[cfg(feature = "stm32f207zg")]
teleprobe_meta::target!(b"nucleo-stm32f207zg");
#[cfg(feature = "stm32f303ze")]
teleprobe_meta::target!(b"nucleo-stm32f303ze");
#[cfg(feature = "stm32l496zg")]
teleprobe_meta::target!(b"nucleo-stm32l496zg");

macro_rules! define_peris {
    ($($name:ident = $peri:ident,)* $(@irq $irq_name:ident = $irq_code:tt,)*) => {
        #[allow(unused_macros)]
        macro_rules! peri {
            $(
                ($p:expr, $name) => {
                    $p.$peri
                };
            )*
        }
        #[allow(unused_macros)]
        macro_rules! irqs {
            $(
                ($irq_name) => {{
                    embassy_stm32::bind_interrupts!(struct Irqs $irq_code);
                    Irqs
                }};
            )*
        }

        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub mod peris {
            $(
                pub type $name = embassy_stm32::peripherals::$peri;
            )*
        }
    };
}

#[cfg(feature = "stm32f103c8")]
define_peris!(
    UART = USART1, UART_TX = PA9, UART_RX = PA10, UART_TX_DMA = DMA1_CH4, UART_RX_DMA = DMA1_CH5,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);
#[cfg(feature = "stm32g491re")]
define_peris!(
    UART = USART1, UART_TX = PC4, UART_RX = PC5, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);
#[cfg(feature = "stm32g071rb")]
define_peris!(
    UART = USART1, UART_TX = PC4, UART_RX = PC5, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);
#[cfg(feature = "stm32f429zi")]
define_peris!(
    UART = USART6, UART_TX = PG14, UART_RX = PG9, UART_TX_DMA = DMA2_CH6, UART_RX_DMA = DMA2_CH1,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA2_CH3, SPI_RX_DMA = DMA2_CH2,
    @irq UART = {USART6 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART6>;},
);
#[cfg(feature = "stm32wb55rg")]
define_peris!(
    UART = LPUART1, UART_TX = PA2, UART_RX = PA3, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {LPUART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::LPUART1>;},
);
#[cfg(feature = "stm32h755zi")]
define_peris!(
    UART = USART1, UART_TX = PB6, UART_RX = PB7, UART_TX_DMA = DMA1_CH0, UART_RX_DMA = DMA1_CH1,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PB5, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH0, SPI_RX_DMA = DMA1_CH1,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);
#[cfg(feature = "stm32u585ai")]
define_peris!(
    UART = USART3, UART_TX = PD8, UART_RX = PD9, UART_TX_DMA = GPDMA1_CH0, UART_RX_DMA = GPDMA1_CH1,
    SPI = SPI1, SPI_SCK = PE13, SPI_MOSI = PE15, SPI_MISO = PE14, SPI_TX_DMA = GPDMA1_CH0, SPI_RX_DMA = GPDMA1_CH1,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32h563zi")]
define_peris!(
    UART = LPUART1, UART_TX = PB6, UART_RX = PB7, UART_TX_DMA = GPDMA1_CH0, UART_RX_DMA = GPDMA1_CH1,
    SPI = SPI4, SPI_SCK = PE12, SPI_MOSI = PE14, SPI_MISO = PE13, SPI_TX_DMA = GPDMA1_CH0, SPI_RX_DMA = GPDMA1_CH1,
    @irq UART = {LPUART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::LPUART1>;},
);
#[cfg(feature = "stm32c031c6")]
define_peris!(
    UART = USART1, UART_TX = PB6, UART_RX = PB7, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);
#[cfg(feature = "stm32l496zg")]
define_peris!(
    UART = USART3, UART_TX = PD8, UART_RX = PD9, UART_TX_DMA = DMA1_CH2, UART_RX_DMA = DMA1_CH3,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32l4a6zg")]
define_peris!(
    UART = USART3, UART_TX = PD8, UART_RX = PD9, UART_TX_DMA = DMA1_CH2, UART_RX_DMA = DMA1_CH3,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32l4r5zi")]
define_peris!(
    UART = USART3, UART_TX = PD8, UART_RX = PD9, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32l073rz")]
define_peris!(
    UART = USART4, UART_TX = PA0, UART_RX = PA1, UART_TX_DMA = DMA1_CH3, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART4_5 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART4>;},
);
#[cfg(feature = "stm32l152re")]
define_peris!(
    UART = USART3, UART_TX = PB10, UART_RX = PB11, UART_TX_DMA = DMA1_CH2, UART_RX_DMA = DMA1_CH3,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32l552ze")]
define_peris!(
    UART = USART3, UART_TX = PD8, UART_RX = PD9, UART_TX_DMA = DMA1_CH1, UART_RX_DMA = DMA1_CH2,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH1, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART3 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART3>;},
);
#[cfg(feature = "stm32f767zi")]
define_peris!(
    UART = USART6, UART_TX = PG14, UART_RX = PG9, UART_TX_DMA = DMA2_CH6, UART_RX_DMA = DMA2_CH1,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA2_CH3, SPI_RX_DMA = DMA2_CH2,
    @irq UART = {USART6 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART6>;},
);
#[cfg(feature = "stm32f207zg")]
define_peris!(
    UART = USART6, UART_TX = PG14, UART_RX = PG9, UART_TX_DMA = DMA2_CH6, UART_RX_DMA = DMA2_CH1,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA2_CH3, SPI_RX_DMA = DMA2_CH2,
    @irq UART = {USART6 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART6>;},
);
#[cfg(feature = "stm32f303ze")]
define_peris!(
    UART = USART1, UART_TX = PC4, UART_RX = PC5, UART_TX_DMA = DMA1_CH4, UART_RX_DMA = DMA1_CH5,
    SPI = SPI1, SPI_SCK = PA5, SPI_MOSI = PA7, SPI_MISO = PA6, SPI_TX_DMA = DMA1_CH3, SPI_RX_DMA = DMA1_CH2,
    @irq UART = {USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;},
);

pub fn config() -> Config {
    #[allow(unused_mut)]
    let mut config = Config::default();

    #[cfg(feature = "stm32f207zg")]
    {
        use embassy_stm32::rcc::*;
        // By default, HSE on the board comes from a 8 MHz clock signal (not a crystal)
        config.rcc.hse = Some(HSEConfig {
            frequency: Hertz(8_000_000),
            source: HSESrc::Bypass,
        });
        // PLL uses HSE as the clock source
        config.rcc.pll_mux = PLLSrc::HSE;
        config.rcc.pll = PLLConfig {
            // 8 MHz clock source / 8 = 1 MHz PLL input
            pre_div: unwrap!(PLLPreDiv::try_from(8)),
            // 1 MHz PLL input * 240 = 240 MHz PLL VCO
            mul: unwrap!(PLLMul::try_from(240)),
            // 240 MHz PLL VCO / 2 = 120 MHz main PLL output
            p_div: PLLPDiv::DIV2,
            // 240 MHz PLL VCO / 5 = 48 MHz PLL48 output
            q_div: PLLQDiv::DIV5,
        };
        // System clock comes from PLL (= the 120 MHz main PLL output)
        config.rcc.mux = ClockSrc::PLL;
        // 120 MHz / 4 = 30 MHz APB1 frequency
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        // 120 MHz / 2 = 60 MHz APB2 frequency
        config.rcc.apb2_pre = APBPrescaler::DIV2;
    }

    #[cfg(feature = "stm32f429zi")]
    {
        // TODO: stm32f429zi can do up to 180mhz, but that makes tests fail.
        // perhaps we have some bug w.r.t overdrive.
        config.rcc.sys_ck = Some(Hertz(168_000_000));
        config.rcc.pclk1 = Some(Hertz(42_000_000));
        config.rcc.pclk2 = Some(Hertz(84_000_000));
    }

    #[cfg(feature = "stm32f767zi")]
    {
        config.rcc.sys_ck = Some(Hertz(200_000_000));
    }

    #[cfg(feature = "stm32h563zi")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = None;
        config.rcc.hsi48 = true; // needed for rng
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::BypassDigital,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL125,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV2),
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV1;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.apb3_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::Pll1P;
        config.rcc.voltage_scale = VoltageScale::Scale0;
    }

    #[cfg(feature = "stm32h755zi")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(Hsi::Mhz64);
        config.rcc.csi = true;
        config.rcc.hsi48 = true; // needed for RNG
        config.rcc.pll_src = PllSource::Hsi;
        config.rcc.pll1 = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV8), // SPI1 cksel defaults to pll1_q
            divr: None,
        });
        config.rcc.pll2 = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV8), // 100mhz
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::Pll1P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
        config.rcc.adc_clock_source = AdcClockSource::PLL2_P;
    }

    #[cfg(any(feature = "stm32l4a6zg", feature = "stm32l4r5zi"))]
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux = ClockSrc::PLL(
            // 72Mhz clock (16 / 1 * 18 / 4)
            PLLSource::HSI16,
            PllRDiv::DIV4,
            PllPreDiv::DIV1,
            PllMul::MUL18,
            Some(PllQDiv::DIV6), // 48Mhz (16 / 1 * 18 / 6)
        );
    }

    #[cfg(any(feature = "stm32l552ze"))]
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux = ClockSrc::PLL(
            // 110Mhz clock (16 / 4 * 55 / 2)
            PLLSource::HSI16,
            PllRDiv::DIV2,
            PllPreDiv::DIV4,
            PllMul::MUL55,
            None,
        );
    }

    #[cfg(feature = "stm32u585ai")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux = ClockSrc::MSI(Msirange::RANGE_48MHZ);
    }

    #[cfg(feature = "stm32l073rz")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux = ClockSrc::PLL(
            // 32Mhz clock (16 * 4 / 2)
            PLLSource::HSI16,
            PLLMul::MUL4,
            PLLDiv::DIV2,
        );
    }

    #[cfg(any(feature = "stm32l152re"))]
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux = ClockSrc::PLL(
            // 32Mhz clock (16 * 4 / 2)
            PLLSource::HSI16,
            PLLMul::MUL4,
            PLLDiv::DIV2,
        );
    }

    config
}
