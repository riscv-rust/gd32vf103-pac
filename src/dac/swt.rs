#[doc = "Register `SWT` writer"]
pub type W = crate::W<SWT_SPEC>;
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type SWTR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTR1` writer - DAC1 software trigger"]
pub type SWTR1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr0(&mut self) -> SWTR0_W<SWT_SPEC> {
        SWTR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr1(&mut self) -> SWTR1_W<SWT_SPEC> {
        SWTR1_W::new(self, 1)
    }
}
#[doc = "software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWT_SPEC;
impl crate::RegisterSpec for SWT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swt::W`](W) writer structure"]
impl crate::Writable for SWT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SWT_SPEC {
    const RESET_VALUE: u32 = 0;
}
