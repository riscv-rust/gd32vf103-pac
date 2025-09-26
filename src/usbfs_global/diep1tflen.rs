#[doc = "Register `DIEP1TFLEN` reader"]
pub type R = crate::R<DIEP1TFLEN_SPEC>;
#[doc = "Register `DIEP1TFLEN` writer"]
pub type W = crate::W<DIEP1TFLEN_SPEC>;
#[doc = "Field `IEPTXRSAR` reader - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_R = crate::FieldReader<u16>;
#[doc = "Field `IEPTXRSAR` writer - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type IEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `IEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type IEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO transmit RAM start address"]
    #[inline(always)]
    pub fn ieptxrsar(&self) -> IEPTXRSAR_R {
        IEPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ieptxfd(&self) -> IEPTXFD_R {
        IEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxrsar(&mut self) -> IEPTXRSAR_W<DIEP1TFLEN_SPEC> {
        IEPTXRSAR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxfd(&mut self) -> IEPTXFD_W<DIEP1TFLEN_SPEC> {
        IEPTXFD_W::new(self, 16)
    }
}
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1tflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1tflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP1TFLEN_SPEC;
impl crate::RegisterSpec for DIEP1TFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep1tflen::R`](R) reader structure"]
impl crate::Readable for DIEP1TFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep1tflen::W`](W) writer structure"]
impl crate::Writable for DIEP1TFLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP1TFLEN to value 0x0200_0400"]
impl crate::Resettable for DIEP1TFLEN_SPEC {
    const RESET_VALUE: u32 = 0x0200_0400;
}
