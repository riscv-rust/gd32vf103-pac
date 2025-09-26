#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `SBKCMD` reader - Send break command"]
pub type SBKCMD_R = crate::BitReader;
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SBKCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - Receiver wakeup from mute mode"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `RWU` writer - Receiver wakeup from mute mode"]
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE line detected interrupt enable"]
pub type IDLEIE_R = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE line detected interrupt enable"]
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEIE` reader - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_R = crate::BitReader;
#[doc = "Field `RBNEIE` writer - Read data buffer not empty interrupt and overrun error interrupt enable"]
pub type RBNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEIE` reader - Transmitter buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader;
#[doc = "Field `TBEIE` writer - Transmitter buffer empty interrupt enable"]
pub type TBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Parity mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Parity mode"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - Parity check function enable"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - Parity check function enable"]
pub type PCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WM` reader - Wakeup method in mute mode"]
pub type WM_R = crate::BitReader;
#[doc = "Field `WM` writer - Wakeup method in mute mode"]
pub type WM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL` reader - Word length"]
pub type WL_R = crate::BitReader;
#[doc = "Field `WL` writer - Word length"]
pub type WL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    pub fn sbkcmd(&self) -> SBKCMD_R {
        SBKCMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break command"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SBKCMD_W<CTL0_SPEC> {
        SBKCMD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver wakeup from mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CTL0_SPEC> {
        RWU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTL0_SPEC> {
        REN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTL0_SPEC> {
        TEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE line detected interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CTL0_SPEC> {
        IDLEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Read data buffer not empty interrupt and overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL0_SPEC> {
        RBNEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CTL0_SPEC> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL0_SPEC> {
        TBEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<CTL0_SPEC> {
        PERRIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<CTL0_SPEC> {
        PM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity check function enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<CTL0_SPEC> {
        PCEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup method in mute mode"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<CTL0_SPEC> {
        WM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WL_W<CTL0_SPEC> {
        WL_W::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTL0_SPEC> {
        UEN_W::new(self, 13)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
