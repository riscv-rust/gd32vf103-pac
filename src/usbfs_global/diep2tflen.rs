#[doc = "Register `DIEP2TFLEN` reader"]
pub type R = crate::R<DIEP2TFLEN_SPEC>;
#[doc = "Register `DIEP2TFLEN` writer"]
pub type W = crate::W<DIEP2TFLEN_SPEC>;
#[doc = "Field `IEPTXRSAR` reader - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_R = crate::FieldReader<u16>;
#[doc = "Field `IEPTXRSAR` writer - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `IEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type IEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `IEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type IEPTXFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn ieptxrsar(&mut self) -> IEPTXRSAR_W<DIEP2TFLEN_SPEC, 0> {
        IEPTXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxfd(&mut self) -> IEPTXFD_W<DIEP2TFLEN_SPEC, 16> {
        IEPTXFD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2tflen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2tflen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP2TFLEN_SPEC;
impl crate::RegisterSpec for DIEP2TFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2tflen::R`](R) reader structure"]
impl crate::Readable for DIEP2TFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep2tflen::W`](W) writer structure"]
impl crate::Writable for DIEP2TFLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP2TFLEN to value 0x0200_0400"]
impl crate::Resettable for DIEP2TFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
