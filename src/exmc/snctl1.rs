#[doc = "Register `SNCTL1` reader"]
pub struct R(crate::R<SNCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNCTL1` writer"]
pub struct W(crate::W<SNCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SNCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRBKEN` reader - NOR bank enable"]
pub type NRBKEN_R = crate::BitReader<bool>;
#[doc = "Field `NRBKEN` writer - NOR bank enable"]
pub type NRBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `NRMUX` reader - NOR bank memory address/data multiplexing"]
pub type NRMUX_R = crate::BitReader<bool>;
#[doc = "Field `NRMUX` writer - NOR bank memory address/data multiplexing"]
pub type NRMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `NRTP` reader - NOR bank memory type"]
pub type NRTP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRTP` writer - NOR bank memory type"]
pub type NRTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNCTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `NRW` reader - NOR bank memory data bus width"]
pub type NRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRW` writer - NOR bank memory data bus width"]
pub type NRW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNCTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `NREN` reader - NOR Flash access enable"]
pub type NREN_R = crate::BitReader<bool>;
#[doc = "Field `NREN` writer - NOR Flash access enable"]
pub type NREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `NRWTPOL` reader - NWAIT signal polarity"]
pub type NRWTPOL_R = crate::BitReader<bool>;
#[doc = "Field `NRWTPOL` writer - NWAIT signal polarity"]
pub type NRWTPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `WREN` reader - Write enable"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Write enable"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `NRWTEN` reader - NWAIT signal enable"]
pub type NRWTEN_R = crate::BitReader<bool>;
#[doc = "Field `NRWTEN` writer - NWAIT signal enable"]
pub type NRWTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
#[doc = "Field `ASYNCWAIT` reader - Asynchronous wait"]
pub type ASYNCWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCWAIT` writer - Asynchronous wait"]
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    pub fn nrbken(&self) -> NRBKEN_R {
        NRBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    pub fn nrmux(&self) -> NRMUX_R {
        NRMUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    pub fn nrtp(&self) -> NRTP_R {
        NRTP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    pub fn nrw(&self) -> NRW_R {
        NRW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    pub fn nren(&self) -> NREN_R {
        NREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    pub fn nrwtpol(&self) -> NRWTPOL_R {
        NRWTPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    pub fn nrwten(&self) -> NRWTEN_R {
        NRWTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NOR bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn nrbken(&mut self) -> NRBKEN_W<0> {
        NRBKEN_W::new(self)
    }
    #[doc = "Bit 1 - NOR bank memory address/data multiplexing"]
    #[inline(always)]
    #[must_use]
    pub fn nrmux(&mut self) -> NRMUX_W<1> {
        NRMUX_W::new(self)
    }
    #[doc = "Bits 2:3 - NOR bank memory type"]
    #[inline(always)]
    #[must_use]
    pub fn nrtp(&mut self) -> NRTP_W<2> {
        NRTP_W::new(self)
    }
    #[doc = "Bits 4:5 - NOR bank memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn nrw(&mut self) -> NRW_W<4> {
        NRW_W::new(self)
    }
    #[doc = "Bit 6 - NOR Flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn nren(&mut self) -> NREN_W<6> {
        NREN_W::new(self)
    }
    #[doc = "Bit 9 - NWAIT signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn nrwtpol(&mut self) -> NRWTPOL_W<9> {
        NRWTPOL_W::new(self)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 13 - NWAIT signal enable"]
    #[inline(always)]
    #[must_use]
    pub fn nrwten(&mut self) -> NRWTEN_W<13> {
        NRWTEN_W::new(self)
    }
    #[doc = "Bit 15 - Asynchronous wait"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR flash control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl1](index.html) module"]
pub struct SNCTL1_SPEC;
impl crate::RegisterSpec for SNCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snctl1::R](R) reader structure"]
impl crate::Readable for SNCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snctl1::W](W) writer structure"]
impl crate::Writable for SNCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNCTL1 to value 0x30da"]
impl crate::Resettable for SNCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x30da;
}
