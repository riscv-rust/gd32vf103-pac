#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CRCPOLY_SPEC>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CRCPOLY_SPEC>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial value"]
pub type CRCPOLY_R = crate::FieldReader<u16>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial value"]
pub type CRCPOLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial value"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial value"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<CRCPOLY_SPEC, 0> {
        CRCPOLY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcpoly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCPOLY_SPEC;
impl crate::RegisterSpec for CRCPOLY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CRCPOLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CRCPOLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCPOLY to value 0x07"]
impl crate::Resettable for CRCPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
