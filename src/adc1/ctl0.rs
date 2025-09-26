#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WDCHSEL_R = crate::FieldReader;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WDCHSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDEIE` reader - Interrupt enable for WDE"]
pub type WDEIE_R = crate::BitReader;
#[doc = "Field `WDEIE` writer - Interrupt enable for WDE"]
pub type WDEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EOICIE_R = crate::BitReader;
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EOICIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SM` reader - Scan mode"]
pub type SM_R = crate::BitReader;
#[doc = "Field `SM` writer - Scan mode"]
pub type SM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_R = crate::BitReader;
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WDSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type ICA_R = crate::BitReader;
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type ICA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DISRC_R = crate::BitReader;
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DISRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted channels"]
pub type DISIC_R = crate::BitReader;
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted channels"]
pub type DISIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DISNUM_R = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DISNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IWDEN_R = crate::BitReader;
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RWDEN_R = crate::BitReader;
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WDCHSEL_R {
        WDCHSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WDEIE_R {
        WDEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EOICIE_R {
        EOICIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WDSC_R {
        WDSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> ICA_R {
        ICA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DISRC_R {
        DISRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&self) -> DISIC_R {
        DISIC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DISNUM_R {
        DISNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IWDEN_R {
        IWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RWDEN_R {
        RWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    #[must_use]
    pub fn wdchsel(&mut self) -> WDCHSEL_W<CTL0_SPEC, 0> {
        WDCHSEL_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CTL0_SPEC, 5> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable for WDE"]
    #[inline(always)]
    #[must_use]
    pub fn wdeie(&mut self) -> WDEIE_W<CTL0_SPEC, 6> {
        WDEIE_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    #[must_use]
    pub fn eoicie(&mut self) -> EOICIE_W<CTL0_SPEC, 7> {
        EOICIE_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<CTL0_SPEC, 8> {
        SM_W::new(self)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn wdsc(&mut self) -> WDSC_W<CTL0_SPEC, 9> {
        WDSC_W::new(self)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    #[must_use]
    pub fn ica(&mut self) -> ICA_W<CTL0_SPEC, 10> {
        ICA_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn disrc(&mut self) -> DISRC_W<CTL0_SPEC, 11> {
        DISRC_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    #[must_use]
    pub fn disic(&mut self) -> DISIC_W<CTL0_SPEC, 12> {
        DISIC_W::new(self)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn disnum(&mut self) -> DISNUM_W<CTL0_SPEC, 13> {
        DISNUM_W::new(self)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwden(&mut self) -> IWDEN_W<CTL0_SPEC, 22> {
        IWDEN_W::new(self)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwden(&mut self) -> RWDEN_W<CTL0_SPEC, 23> {
        RWDEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
