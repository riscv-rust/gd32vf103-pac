#[doc = "Register `HPTFLEN` reader"]
pub type R = crate::R<HPTFLEN_SPEC>;
#[doc = "Register `HPTFLEN` writer"]
pub type W = crate::W<HPTFLEN_SPEC>;
#[doc = "Field `HPTXFSAR` reader - Host periodic TxFIFO start address"]
pub type HPTXFSAR_R = crate::FieldReader<u16>;
#[doc = "Field `HPTXFSAR` writer - Host periodic TxFIFO start address"]
pub type HPTXFSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `HPTXFD` reader - Host periodic TxFIFO depth"]
pub type HPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `HPTXFD` writer - Host periodic TxFIFO depth"]
pub type HPTXFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&self) -> HPTXFSAR_R {
        HPTXFSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&self) -> HPTXFD_R {
        HPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn hptxfsar(&mut self) -> HPTXFSAR_W<HPTFLEN_SPEC, 0> {
        HPTXFSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn hptxfd(&mut self) -> HPTXFD_W<HPTFLEN_SPEC, 16> {
        HPTXFD_W::new(self)
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
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptflen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptflen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTFLEN_SPEC;
impl crate::RegisterSpec for HPTFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptflen::R`](R) reader structure"]
impl crate::Readable for HPTFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptflen::W`](W) writer structure"]
impl crate::Writable for HPTFLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTFLEN to value 0x0200_0600"]
impl crate::Resettable for HPTFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0600;
}
