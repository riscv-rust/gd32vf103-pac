#[doc = "Register `CID` reader"]
pub type R = crate::R<CID_SPEC>;
#[doc = "Register `CID` writer"]
pub type W = crate::W<CID_SPEC>;
#[doc = "Field `CID` reader - Core ID"]
pub type CID_R = crate::FieldReader<u32>;
#[doc = "Field `CID` writer - Core ID"]
pub type CID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Core ID"]
    #[inline(always)]
    pub fn cid(&self) -> CID_R {
        CID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core ID"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CID_W<CID_SPEC, 0> {
        CID_W::new(self)
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
#[doc = "core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID_SPEC;
impl crate::RegisterSpec for CID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid::R`](R) reader structure"]
impl crate::Readable for CID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CID to value 0x1000"]
impl crate::Resettable for CID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
