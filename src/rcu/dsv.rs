#[doc = "Register `DSV` reader"]
pub type R = crate::R<DSV_SPEC>;
#[doc = "Register `DSV` writer"]
pub type W = crate::W<DSV_SPEC>;
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DSLPVS_R = crate::FieldReader;
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DSLPVS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DSLPVS_R {
        DSLPVS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn dslpvs(&mut self) -> DSLPVS_W<DSV_SPEC> {
        DSLPVS_W::new(self, 0)
    }
}
#[doc = "Deep sleep mode Voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSV_SPEC;
impl crate::RegisterSpec for DSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv::R`](R) reader structure"]
impl crate::Readable for DSV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsv::W`](W) writer structure"]
impl crate::Writable for DSV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DSV_SPEC {
    const RESET_VALUE: u32 = 0;
}
