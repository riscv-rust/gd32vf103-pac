#[doc = "Register `RFIFO0` reader"]
pub struct R(crate::R<RFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO0` writer"]
pub struct W(crate::W<RFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO0_SPEC>;
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
impl From<crate::W<RFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFL0` reader - Receive FIFO0 length"]
pub type RFL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFF0` reader - Receive FIFO0 full"]
pub type RFF0_R = crate::BitReader<bool>;
#[doc = "Field `RFF0` writer - Receive FIFO0 full"]
pub type RFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, O>;
#[doc = "Field `RFO0` reader - Receive FIFO0 overfull"]
pub type RFO0_R = crate::BitReader<bool>;
#[doc = "Field `RFO0` writer - Receive FIFO0 overfull"]
pub type RFO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, O>;
#[doc = "Field `RFD0` reader - Receive FIFO0 dequeue"]
pub type RFD0_R = crate::BitReader<bool>;
#[doc = "Field `RFD0` writer - Receive FIFO0 dequeue"]
pub type RFD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO0 length"]
    #[inline(always)]
    pub fn rfl0(&self) -> RFL0_R {
        RFL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    pub fn rff0(&self) -> RFF0_R {
        RFF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    pub fn rfo0(&self) -> RFO0_R {
        RFO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&self) -> RFD0_R {
        RFD0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff0(&mut self) -> RFF0_W<3> {
        RFF0_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo0(&mut self) -> RFO0_W<4> {
        RFO0_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd0(&mut self) -> RFD0_W<5> {
        RFD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive message FIFO0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo0](index.html) module"]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo0::R](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo0::W](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
