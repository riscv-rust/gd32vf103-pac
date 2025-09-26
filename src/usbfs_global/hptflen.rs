#[doc = "Register `HPTFLEN` reader"]
pub type R = crate::R<HPTFLEN_SPEC>;
#[doc = "Register `HPTFLEN` writer"]
pub type W = crate::W<HPTFLEN_SPEC>;
#[doc = "Field `HPTXFSAR` reader - Host periodic TxFIFO start address"]
pub type HPTXFSAR_R = crate::FieldReader<u16>;
#[doc = "Field `HPTXFSAR` writer - Host periodic TxFIFO start address"]
pub type HPTXFSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HPTXFD` reader - Host periodic TxFIFO depth"]
pub type HPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `HPTXFD` writer - Host periodic TxFIFO depth"]
pub type HPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn hptxfsar(&mut self) -> HPTXFSAR_W<HPTFLEN_SPEC> {
        HPTXFSAR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn hptxfd(&mut self) -> HPTXFD_W<HPTFLEN_SPEC> {
        HPTXFD_W::new(self, 16)
    }
}
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`hptflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTFLEN_SPEC;
impl crate::RegisterSpec for HPTFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptflen::R`](R) reader structure"]
impl crate::Readable for HPTFLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptflen::W`](W) writer structure"]
impl crate::Writable for HPTFLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTFLEN to value 0x0200_0600"]
impl crate::Resettable for HPTFLEN_SPEC {
    const RESET_VALUE: u32 = 0x0200_0600;
}
