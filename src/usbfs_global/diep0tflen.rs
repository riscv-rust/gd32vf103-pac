#[doc = "Register `DIEP0TFLEN` reader"]
pub type R = crate::R<DIEP0TFLEN_SPEC>;
#[doc = "Register `DIEP0TFLEN` writer"]
pub type W = crate::W<DIEP0TFLEN_SPEC>;
#[doc = "Field `IEP0TXRSAR` reader - in endpoint 0 Tx RAM start address"]
pub type IEP0TXRSAR_R = crate::FieldReader<u16>;
#[doc = "Field `IEP0TXRSAR` writer - in endpoint 0 Tx RAM start address"]
pub type IEP0TXRSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `IEP0TXFD` reader - in endpoint 0 Tx FIFO depth"]
pub type IEP0TXFD_R = crate::FieldReader<u16>;
#[doc = "Field `IEP0TXFD` writer - in endpoint 0 Tx FIFO depth"]
pub type IEP0TXFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    pub fn iep0txrsar(&self) -> IEP0TXRSAR_R {
        IEP0TXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    pub fn iep0txfd(&self) -> IEP0TXFD_R {
        IEP0TXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn iep0txrsar(&mut self) -> IEP0TXRSAR_W<DIEP0TFLEN_SPEC, 0> {
        IEP0TXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn iep0txfd(&mut self) -> IEP0TXFD_W<DIEP0TFLEN_SPEC, 16> {
        IEP0TXFD_W::new(self)
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
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tflen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0tflen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0TFLEN_SPEC;
impl crate::RegisterSpec for DIEP0TFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0tflen::R`](R) reader structure"]
impl crate::Readable for DIEP0TFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0tflen::W`](W) writer structure"]
impl crate::Writable for DIEP0TFLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0TFLEN to value 0x0200_0200"]
impl crate::Resettable for DIEP0TFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0200;
}
