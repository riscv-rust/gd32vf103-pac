#[doc = "Register `RFIFO0` reader"]
pub type R = crate::R<RFIFO0_SPEC>;
#[doc = "Register `RFIFO0` writer"]
pub type W = crate::W<RFIFO0_SPEC>;
#[doc = "Field `RFL0` reader - Receive FIFO0 length"]
pub type RFL0_R = crate::FieldReader;
#[doc = "Field `RFF0` reader - Receive FIFO0 full"]
pub type RFF0_R = crate::BitReader;
#[doc = "Field `RFF0` writer - Receive FIFO0 full"]
pub type RFF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFO0` reader - Receive FIFO0 overfull"]
pub type RFO0_R = crate::BitReader;
#[doc = "Field `RFO0` writer - Receive FIFO0 overfull"]
pub type RFO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFD0` reader - Receive FIFO0 dequeue"]
pub type RFD0_R = crate::BitReader;
#[doc = "Field `RFD0` writer - Receive FIFO0 dequeue"]
pub type RFD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn rff0(&mut self) -> RFF0_W<RFIFO0_SPEC, 3> {
        RFF0_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo0(&mut self) -> RFO0_W<RFIFO0_SPEC, 4> {
        RFO0_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd0(&mut self) -> RFD0_W<RFIFO0_SPEC, 5> {
        RFD0_W::new(self)
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
#[doc = "Receive message FIFO0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo0::R`](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo0::W`](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
