#[doc = "Register `SWT` writer"]
pub type W = crate::W<SWT_SPEC>;
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type SWTR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTR1` writer - DAC1 software trigger"]
pub type SWTR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr0(&mut self) -> SWTR0_W<SWT_SPEC, 0> {
        SWTR0_W::new(self)
    }
    #[doc = "Bit 1 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr1(&mut self) -> SWTR1_W<SWT_SPEC, 1> {
        SWTR1_W::new(self)
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
#[doc = "software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWT_SPEC;
impl crate::RegisterSpec for SWT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swt::W`](W) writer structure"]
impl crate::Writable for SWT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SWT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
