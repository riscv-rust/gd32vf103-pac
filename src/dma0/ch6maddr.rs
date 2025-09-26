#[doc = "Register `CH6MADDR` reader"]
pub type R = crate::R<CH6MADDR_SPEC>;
#[doc = "Register `CH6MADDR` writer"]
pub type W = crate::W<CH6MADDR_SPEC>;
#[doc = "Field `MADDR` reader - Memory base address"]
pub type MADDR_R = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory base address"]
pub type MADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&self) -> MADDR_R {
        MADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MADDR_W<CH6MADDR_SPEC> {
        MADDR_W::new(self, 0)
    }
}
#[doc = "Channel 6 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6MADDR_SPEC;
impl crate::RegisterSpec for CH6MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6maddr::R`](R) reader structure"]
impl crate::Readable for CH6MADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch6maddr::W`](W) writer structure"]
impl crate::Writable for CH6MADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6MADDR to value 0"]
impl crate::Resettable for CH6MADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
