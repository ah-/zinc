INCLUDE ./src/hal/cortex_m3/armmem.ld

PROVIDE(isr_wdt           = isr_default_fault);
PROVIDE(isr_timer_0       = isr_default_fault);
PROVIDE(isr_timer_1       = isr_default_fault);
PROVIDE(isr_timer_2       = isr_default_fault);
PROVIDE(isr_timer_3       = isr_default_fault);
PROVIDE(isr_uart_0        = isr_default_fault);
PROVIDE(isr_uart_1        = isr_default_fault);
PROVIDE(isr_uart_2        = isr_default_fault);
PROVIDE(isr_uart_3        = isr_default_fault);
PROVIDE(isr_pwm_1         = isr_default_fault);
PROVIDE(isr_i2c_0         = isr_default_fault);
PROVIDE(isr_i2c_1         = isr_default_fault);
PROVIDE(isr_i2c_2         = isr_default_fault);
PROVIDE(isr_spi           = isr_default_fault);
PROVIDE(isr_ssp_0         = isr_default_fault);
PROVIDE(isr_ssp_1         = isr_default_fault);
PROVIDE(isr_pll_0         = isr_default_fault);
PROVIDE(isr_rtc           = isr_default_fault);
PROVIDE(isr_eint_0        = isr_default_fault);
PROVIDE(isr_eint_1        = isr_default_fault);
PROVIDE(isr_eint_2        = isr_default_fault);
PROVIDE(isr_eint_3        = isr_default_fault);
PROVIDE(isr_adc           = isr_default_fault);
PROVIDE(isr_bod           = isr_default_fault);
PROVIDE(isr_usb           = isr_default_fault);
PROVIDE(isr_can           = isr_default_fault);
PROVIDE(isr_dma           = isr_default_fault);
PROVIDE(isr_i2s           = isr_default_fault);
PROVIDE(isr_enet          = isr_default_fault);
PROVIDE(isr_rit           = isr_default_fault);
PROVIDE(isr_mcpwm         = isr_default_fault);
PROVIDE(isr_qei           = isr_default_fault);
PROVIDE(isr_pll_1         = isr_default_fault);
PROVIDE(isr_usb_activity  = isr_default_fault);
PROVIDE(isr_can_activity  = isr_default_fault);

iomem_GPIO0     = 0x2009C000;
iomem_GPIO1     = 0x2009C020;
iomem_GPIO2     = 0x2009C040;
iomem_GPIO3     = 0x2009C060;
iomem_GPIO4     = 0x2009C080;

iomem_TIMER0    = 0x40004000;
iomem_TIMER1    = 0x40008000;

iomem_UART0     = 0x4000C000;

iomem_PINSEL0   = 0x4002C000;
iomem_PINSEL1   = 0x4002C004;
iomem_PINSEL2   = 0x4002C008;
iomem_PINSEL3   = 0x4002C00C;
iomem_PINSEL4   = 0x4002C010;
iomem_PINSEL7   = 0x4002C01C;
iomem_PINSEL9   = 0x4002C024;
iomem_PINSEL10  = 0x4002C028;

iomem_SSP1      = 0x40030000;
iomem_SSP0      = 0x40088000;

iomem_TIMER2    = 0x40090000;
iomem_TIMER3    = 0x40094000;

iomem_UART2     = 0x40098000;
iomem_UART3     = 0x4009C000;

iomem_FLASHCFG  = 0x400FC000;

iomem_PLL0CON   = 0x400FC080;
iomem_PLL0CFG   = 0x400FC084;
iomem_PLL0STAT  = 0x400FC088;
iomem_PLL0FEED  = 0x400FC08C;

iomem_PCONP     = 0x400FC0C4;

iomem_CCLKCFG   = 0x400FC104;
iomem_CLKSRCSEL = 0x400FC10C;

iomem_PCLKSEL0  = 0x400FC1A8;
iomem_PCLKSEL1  = 0x400FC1AC;

iomem_SCS       = 0x400FC1A0;
