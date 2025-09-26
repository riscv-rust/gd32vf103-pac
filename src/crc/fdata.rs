#[doc = "Register `FDATA` reader"]
pub type R = crate::R<FDATA_SPEC>;
#[doc = "Register `FDATA` writer"]
pub type W = crate::W<FDATA_SPEC>;
#[doc = "Field `FDATA` reader - Free Data Register bits"]
pub type FDATA_R = crate::FieldReader;
#[doc = "Field `FDATA` writer - Free Data Register bits"]
pub type FDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&self) -> FDATA_R {
        FDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    #[must_use]
    pub fn fdata(&mut self) -> FDATA_W<FDATA_SPEC> {
        FDATA_W::new(self, 0)
    }
}
#[doc = "Free data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDATA_SPEC;
impl crate::RegisterSpec for FDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdata::R`](R) reader structure"]
impl crate::Readable for FDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdata::W`](W) writer structure"]
impl crate::Writable for FDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDATA to value 0"]
impl crate::Resettable for FDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
