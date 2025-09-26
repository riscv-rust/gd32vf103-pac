#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type DEN0_R = crate::BitReader;
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type DEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type DBOFF0_R = crate::BitReader;
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type DBOFF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type DTEN0_R = crate::BitReader;
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type DTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type DTSEL0_R = crate::FieldReader;
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type DTSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWM0` reader - DAC0 noise wave mode"]
pub type DWM0_R = crate::FieldReader;
#[doc = "Field `DWM0` writer - DAC0 noise wave mode"]
pub type DWM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DWBW0` reader - DAC0 noise wave bit width"]
pub type DWBW0_R = crate::FieldReader;
#[doc = "Field `DWBW0` writer - DAC0 noise wave bit width"]
pub type DWBW0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type DDMAEN0_R = crate::BitReader;
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type DDMAEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub type DEN1_R = crate::BitReader;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub type DEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub type DBOFF1_R = crate::BitReader;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub type DBOFF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub type DTEN1_R = crate::BitReader;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub type DTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub type DTSEL1_R = crate::FieldReader;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub type DTSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWM1` reader - DAC1 noise wave mode"]
pub type DWM1_R = crate::FieldReader;
#[doc = "Field `DWM1` writer - DAC1 noise wave mode"]
pub type DWM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DWBW1` reader - DAC1 noise wave bit width"]
pub type DWBW1_R = crate::FieldReader;
#[doc = "Field `DWBW1` writer - DAC1 noise wave bit width"]
pub type DWBW1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub type DDMAEN1_R = crate::BitReader;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub type DDMAEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> DBOFF0_R {
        DBOFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> DTSEL0_R {
        DTSEL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    pub fn dwm0(&self) -> DWM0_R {
        DWM0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw0(&self) -> DWBW0_R {
        DWBW0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> DDMAEN0_R {
        DDMAEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> DBOFF1_R {
        DBOFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> DTSEL1_R {
        DTSEL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    pub fn dwm1(&self) -> DWM1_R {
        DWM1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    pub fn dwbw1(&self) -> DWBW1_R {
        DWBW1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> DDMAEN1_R {
        DDMAEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> DEN0_W<CTL_SPEC> {
        DEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff0(&mut self) -> DBOFF0_W<CTL_SPEC> {
        DBOFF0_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> DTEN0_W<CTL_SPEC> {
        DTEN0_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0(&mut self) -> DTSEL0_W<CTL_SPEC> {
        DTSEL0_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC0 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm0(&mut self) -> DWM0_W<CTL_SPEC> {
        DWM0_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC0 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw0(&mut self) -> DWBW0_W<CTL_SPEC> {
        DWBW0_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> DDMAEN0_W<CTL_SPEC> {
        DDMAEN0_W::new(self, 12)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den1(&mut self) -> DEN1_W<CTL_SPEC> {
        DEN1_W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff1(&mut self) -> DBOFF1_W<CTL_SPEC> {
        DBOFF1_W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten1(&mut self) -> DTEN1_W<CTL_SPEC> {
        DTEN1_W::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel1(&mut self) -> DTSEL1_W<CTL_SPEC> {
        DTSEL1_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC1 noise wave mode"]
    #[inline(always)]
    #[must_use]
    pub fn dwm1(&mut self) -> DWM1_W<CTL_SPEC> {
        DWM1_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC1 noise wave bit width"]
    #[inline(always)]
    #[must_use]
    pub fn dwbw1(&mut self) -> DWBW1_W<CTL_SPEC> {
        DWBW1_W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen1(&mut self) -> DDMAEN1_W<CTL_SPEC> {
        DDMAEN1_W::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
