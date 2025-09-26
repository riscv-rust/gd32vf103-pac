#[doc = "Register `GRFLEN` reader"]
pub type R = crate::R<GRFLEN_SPEC>;
#[doc = "Register `GRFLEN` writer"]
pub type W = crate::W<GRFLEN_SPEC>;
#[doc = "Field `RXFD` reader - Rx FIFO depth"]
pub type RXFD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - Rx FIFO depth"]
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Rx FIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<GRFLEN_SPEC> {
        RXFD_W::new(self, 0)
    }
}
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`grflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRFLEN_SPEC;
impl crate::RegisterSpec for GRFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grflen::R`](R) reader structure"]
impl crate::Readable for GRFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grflen::W`](W) writer structure"]
impl crate::Writable for GRFLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRFLEN to value 0x0200"]
impl crate::Resettable for GRFLEN_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
