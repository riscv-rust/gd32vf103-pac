#[doc = "Register `HCH0INTF` reader"]
pub struct R(crate::R<HCH0INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCH0INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCH0INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCH0INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCH0INTF` writer"]
pub struct W(crate::W<HCH0INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCH0INTF_SPEC>;
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
impl From<crate::W<HCH0INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCH0INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `CH` reader - Channel halted"]
pub type CH_R = crate::BitReader<bool>;
#[doc = "Field `CH` writer - Channel halted"]
pub type CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `USBER` reader - USB bus error"]
pub type USBER_R = crate::BitReader<bool>;
#[doc = "Field `USBER` writer - USB bus error"]
pub type USBER_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `BBER` reader - Babble error"]
pub type BBER_R = crate::BitReader<bool>;
#[doc = "Field `BBER` writer - Babble error"]
pub type BBER_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `REQOVR` reader - Request queue overrun"]
pub type REQOVR_R = crate::BitReader<bool>;
#[doc = "Field `REQOVR` writer - Request queue overrun"]
pub type REQOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
#[doc = "Field `DTER` reader - Data toggle error"]
pub type DTER_R = crate::BitReader<bool>;
#[doc = "Field `DTER` writer - Data toggle error"]
pub type DTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH0INTF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    pub fn usber(&self) -> USBER_R {
        USBER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bber(&self) -> BBER_R {
        BBER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    pub fn reqovr(&self) -> REQOVR_R {
        REQOVR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dter(&self) -> DTER_R {
        DTER_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<0> {
        TF_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<1> {
        CH_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn usber(&mut self) -> USBER_W<7> {
        USBER_W::new(self)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bber(&mut self) -> BBER_W<8> {
        BBER_W::new(self)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    #[must_use]
    pub fn reqovr(&mut self) -> REQOVR_W<9> {
        REQOVR_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dter(&mut self) -> DTER_W<10> {
        DTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch0intf](index.html) module"]
pub struct HCH0INTF_SPEC;
impl crate::RegisterSpec for HCH0INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hch0intf::R](R) reader structure"]
impl crate::Readable for HCH0INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hch0intf::W](W) writer structure"]
impl crate::Writable for HCH0INTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH0INTF to value 0"]
impl crate::Resettable for HCH0INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
