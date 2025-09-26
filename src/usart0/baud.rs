#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BAUD_SPEC>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BAUD_SPEC>;
#[doc = "Field `FRADIV` reader - Fraction part of baud-rate divider"]
pub type FRADIV_R = crate::FieldReader;
#[doc = "Field `FRADIV` writer - Fraction part of baud-rate divider"]
pub type FRADIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTDIV` reader - Integer part of baud-rate divider"]
pub type INTDIV_R = crate::FieldReader<u16>;
#[doc = "Field `INTDIV` writer - Integer part of baud-rate divider"]
pub type INTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Fraction part of baud-rate divider"]
    #[inline(always)]
    pub fn fradiv(&self) -> FRADIV_R {
        FRADIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Integer part of baud-rate divider"]
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fraction part of baud-rate divider"]
    #[inline(always)]
    #[must_use]
    pub fn fradiv(&mut self) -> FRADIV_W<BAUD_SPEC> {
        FRADIV_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - Integer part of baud-rate divider"]
    #[inline(always)]
    #[must_use]
    pub fn intdiv(&mut self) -> INTDIV_W<BAUD_SPEC> {
        INTDIV_W::new(self, 4)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: u32 = 0;
}
