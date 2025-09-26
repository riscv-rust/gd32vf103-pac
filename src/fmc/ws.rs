#[doc = "Register `WS` reader"]
pub type R = crate::R<WS_SPEC>;
#[doc = "Register `WS` writer"]
pub type W = crate::W<WS_SPEC>;
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WSCNT_R = crate::FieldReader;
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WSCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WSCNT_W<WS_SPEC> {
        WSCNT_W::new(self, 0)
    }
}
#[doc = "wait state counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ws::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WS_SPEC;
impl crate::RegisterSpec for WS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ws::R`](R) reader structure"]
impl crate::Readable for WS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ws::W`](W) writer structure"]
impl crate::Writable for WS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WS to value 0"]
impl crate::Resettable for WS_SPEC {
    const RESET_VALUE: u32 = 0;
}
