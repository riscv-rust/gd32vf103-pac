#[doc = "Register `MTH` reader"]
pub type R = crate::R<MTH_SPEC>;
#[doc = "Register `MTH` writer"]
pub type W = crate::W<MTH_SPEC>;
#[doc = "Field `MTH` reader - MTH"]
pub type MTH_R = crate::FieldReader;
#[doc = "Field `MTH` writer - MTH"]
pub type MTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MTH"]
    #[inline(always)]
    pub fn mth(&self) -> MTH_R {
        MTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MTH"]
    #[inline(always)]
    #[must_use]
    pub fn mth(&mut self) -> MTH_W<MTH_SPEC> {
        MTH_W::new(self, 0)
    }
}
#[doc = "MTH Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTH_SPEC;
impl crate::RegisterSpec for MTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mth::R`](R) reader structure"]
impl crate::Readable for MTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mth::W`](W) writer structure"]
impl crate::Writable for MTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MTH to value 0"]
impl crate::Resettable for MTH_SPEC {
    const RESET_VALUE: u8 = 0;
}
