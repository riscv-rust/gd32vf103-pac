#[doc = "Register `DATA` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `TRB` reader - Transmission or reception data buffer register"]
pub type TRB_R = crate::FieldReader;
#[doc = "Field `TRB` writer - Transmission or reception data buffer register"]
pub type TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmission or reception data buffer register"]
    #[inline(always)]
    pub fn trb(&self) -> TRB_R {
        TRB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmission or reception data buffer register"]
    #[inline(always)]
    #[must_use]
    pub fn trb(&mut self) -> TRB_W<DATA_SPEC> {
        TRB_W::new(self, 0)
    }
}
#[doc = "Transfer buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u16 = 0;
}
