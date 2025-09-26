#[doc = "Register `OBKEY` writer"]
pub type W = crate::W<OBKEY_SPEC>;
#[doc = "Field `OBKEY` writer - FMC_ CTL0 option byte operation unlock register"]
pub type OBKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_ CTL0 option byte operation unlock register"]
    #[inline(always)]
    #[must_use]
    pub fn obkey(&mut self) -> OBKEY_W<OBKEY_SPEC> {
        OBKEY_W::new(self, 0)
    }
}
#[doc = "Option byte unlock key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBKEY_SPEC;
impl crate::RegisterSpec for OBKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`obkey::W`](W) writer structure"]
impl crate::Writable for OBKEY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBKEY to value 0"]
impl crate::Resettable for OBKEY_SPEC {
    const RESET_VALUE: u32 = 0;
}
