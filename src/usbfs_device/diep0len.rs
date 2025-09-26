#[doc = "Register `DIEP0LEN` reader"]
pub type R = crate::R<DIEP0LEN_SPEC>;
#[doc = "Register `DIEP0LEN` writer"]
pub type W = crate::W<DIEP0LEN_SPEC>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<DIEP0LEN_SPEC, 0> {
        TLEN_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<DIEP0LEN_SPEC, 19> {
        PCNT_W::new(self)
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
#[doc = "device IN endpoint-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0LEN_SPEC;
impl crate::RegisterSpec for DIEP0LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0len::R`](R) reader structure"]
impl crate::Readable for DIEP0LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0len::W`](W) writer structure"]
impl crate::Writable for DIEP0LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0LEN to value 0"]
impl crate::Resettable for DIEP0LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
