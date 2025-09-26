#[doc = "Register `CLICINTIE` reader"]
pub type R = crate::R<CLICINTIE_SPEC>;
#[doc = "Register `CLICINTIE` writer"]
pub type W = crate::W<CLICINTIE_SPEC>;
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CLICINTIE_SPEC> {
        IE_W::new(self, 0)
    }
}
#[doc = "clicintie Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTIE_SPEC;
impl crate::RegisterSpec for CLICINTIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintie::R`](R) reader structure"]
impl crate::Readable for CLICINTIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintie::W`](W) writer structure"]
impl crate::Writable for CLICINTIE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLICINTIE to value 0"]
impl crate::Resettable for CLICINTIE_SPEC {
    const RESET_VALUE: u8 = 0;
}
