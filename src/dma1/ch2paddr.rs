#[doc = "Register `CH2PADDR` reader"]
pub type R = crate::R<CH2PADDR_SPEC>;
#[doc = "Register `CH2PADDR` writer"]
pub type W = crate::W<CH2PADDR_SPEC>;
#[doc = "Field `PADDR` reader - Peripheral base address"]
pub type PADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral base address"]
pub type PADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PADDR_W<CH2PADDR_SPEC> {
        PADDR_W::new(self, 0)
    }
}
#[doc = "Channel 2 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2PADDR_SPEC;
impl crate::RegisterSpec for CH2PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2paddr::R`](R) reader structure"]
impl crate::Readable for CH2PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2paddr::W`](W) writer structure"]
impl crate::Writable for CH2PADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2PADDR to value 0"]
impl crate::Resettable for CH2PADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
