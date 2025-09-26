#[doc = "Register `HNPTFLEN` reader"]
pub type R = crate::R<HNPTFLEN_SPEC>;
#[doc = "Register `HNPTFLEN` writer"]
pub type W = crate::W<HNPTFLEN_SPEC>;
#[doc = "Field `HNPTXRSAR` reader - host non-periodic transmit Tx RAM start address"]
pub type HNPTXRSAR_R = crate::FieldReader<u16>;
#[doc = "Field `HNPTXRSAR` writer - host non-periodic transmit Tx RAM start address"]
pub type HNPTXRSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `HNPTXFD` reader - host non-periodic TxFIFO depth"]
pub type HNPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `HNPTXFD` writer - host non-periodic TxFIFO depth"]
pub type HNPTXFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    pub fn hnptxrsar(&self) -> HNPTXRSAR_R {
        HNPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hnptxfd(&self) -> HNPTXFD_R {
        HNPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn hnptxrsar(&mut self) -> HNPTXRSAR_W<HNPTFLEN_SPEC, 0> {
        HNPTXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn hnptxfd(&mut self) -> HNPTXFD_W<HNPTFLEN_SPEC, 16> {
        HNPTXFD_W::new(self)
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
#[doc = "Host non-periodic transmit FIFO length register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptflen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptflen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HNPTFLEN_SPEC;
impl crate::RegisterSpec for HNPTFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hnptflen::R`](R) reader structure"]
impl crate::Readable for HNPTFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hnptflen::W`](W) writer structure"]
impl crate::Writable for HNPTFLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HNPTFLEN to value 0x0200_0200"]
impl crate::Resettable for HNPTFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0200;
}
