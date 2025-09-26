#[doc = "Register `TMP2` reader"]
pub type R = crate::R<TMP2_SPEC>;
#[doc = "Register `TMP2` writer"]
pub type W = crate::W<TMP2_SPEC>;
#[doc = "Field `DLENC` reader - Data length code"]
pub type DLENC_R = crate::FieldReader;
#[doc = "Field `DLENC` writer - Data length code"]
pub type DLENC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TSEN` reader - Time stamp enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Time stamp enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TS` reader - Time stamp"]
pub type TS_R = crate::FieldReader<u16>;
#[doc = "Field `TS` writer - Time stamp"]
pub type TS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DLENC_R {
        DLENC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlenc(&mut self) -> DLENC_W<TMP2_SPEC, 0> {
        DLENC_W::new(self)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<TMP2_SPEC, 8> {
        TSEN_W::new(self)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<TMP2_SPEC, 16> {
        TS_W::new(self)
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
#[doc = "Transmit mailbox property register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMP2_SPEC;
impl crate::RegisterSpec for TMP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmp2::R`](R) reader structure"]
impl crate::Readable for TMP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmp2::W`](W) writer structure"]
impl crate::Writable for TMP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMP2 to value 0"]
impl crate::Resettable for TMP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
