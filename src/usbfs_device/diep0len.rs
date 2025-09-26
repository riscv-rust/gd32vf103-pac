#[doc = "Register `DIEP0LEN` reader"]
pub type R = crate::R<DIEP0LEN_SPEC>;
#[doc = "Register `DIEP0LEN` writer"]
pub type W = crate::W<DIEP0LEN_SPEC>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    pub fn tlen(&mut self) -> TLEN_W<DIEP0LEN_SPEC> {
        TLEN_W::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<DIEP0LEN_SPEC> {
        PCNT_W::new(self, 19)
    }
}
#[doc = "device IN endpoint-0 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0LEN_SPEC;
impl crate::RegisterSpec for DIEP0LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0len::R`](R) reader structure"]
impl crate::Readable for DIEP0LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0len::W`](W) writer structure"]
impl crate::Writable for DIEP0LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP0LEN to value 0"]
impl crate::Resettable for DIEP0LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
