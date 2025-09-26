#[doc = "Register `CLICCFG` reader"]
pub type R = crate::R<CLICCFG_SPEC>;
#[doc = "Register `CLICCFG` writer"]
pub type W = crate::W<CLICCFG_SPEC>;
#[doc = "Field `NLBITS` reader - NLBITS"]
pub type NLBITS_R = crate::FieldReader;
#[doc = "Field `NLBITS` writer - NLBITS"]
pub type NLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    pub fn nlbits(&self) -> NLBITS_R {
        NLBITS_R::new((self.bits >> 1) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    #[must_use]
    pub fn nlbits(&mut self) -> NLBITS_W<CLICCFG_SPEC> {
        NLBITS_W::new(self, 1)
    }
}
#[doc = "cliccfg Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cliccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICCFG_SPEC;
impl crate::RegisterSpec for CLICCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cliccfg::R`](R) reader structure"]
impl crate::Readable for CLICCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cliccfg::W`](W) writer structure"]
impl crate::Writable for CLICCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLICCFG to value 0"]
impl crate::Resettable for CLICCFG_SPEC {
    const RESET_VALUE: u8 = 0;
}
