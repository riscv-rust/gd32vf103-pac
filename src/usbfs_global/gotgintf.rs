#[doc = "Register `GOTGINTF` reader"]
pub type R = crate::R<GOTGINTF_SPEC>;
#[doc = "Register `GOTGINTF` writer"]
pub type W = crate::W<GOTGINTF_SPEC>;
#[doc = "Field `SESEND` reader - Session end"]
pub type SESEND_R = crate::BitReader;
#[doc = "Field `SESEND` writer - Session end"]
pub type SESEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRPEND` reader - Session request success status change"]
pub type SRPEND_R = crate::BitReader;
#[doc = "Field `SRPEND` writer - Session request success status change"]
pub type SRPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPEND` reader - HNP end"]
pub type HNPEND_R = crate::BitReader;
#[doc = "Field `HNPEND` writer - HNP end"]
pub type HNPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPDET` reader - Host negotiation request detected"]
pub type HNPDET_R = crate::BitReader;
#[doc = "Field `HNPDET` writer - Host negotiation request detected"]
pub type HNPDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADTO` reader - A-device timeout"]
pub type ADTO_R = crate::BitReader;
#[doc = "Field `ADTO` writer - A-device timeout"]
pub type ADTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DF` reader - Debounce finish"]
pub type DF_R = crate::BitReader;
#[doc = "Field `DF` writer - Debounce finish"]
pub type DF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&self) -> SESEND_R {
        SESEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srpend(&self) -> SRPEND_R {
        SRPEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&self) -> HNPEND_R {
        HNPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    pub fn hnpdet(&self) -> HNPDET_R {
        HNPDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&self) -> ADTO_R {
        ADTO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&self) -> DF_R {
        DF_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    #[must_use]
    pub fn sesend(&mut self) -> SESEND_W<GOTGINTF_SPEC, 2> {
        SESEND_W::new(self)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    #[must_use]
    pub fn srpend(&mut self) -> SRPEND_W<GOTGINTF_SPEC, 8> {
        SRPEND_W::new(self)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    #[must_use]
    pub fn hnpend(&mut self) -> HNPEND_W<GOTGINTF_SPEC, 9> {
        HNPEND_W::new(self)
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    #[must_use]
    pub fn hnpdet(&mut self) -> HNPDET_W<GOTGINTF_SPEC, 17> {
        HNPDET_W::new(self)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    #[must_use]
    pub fn adto(&mut self) -> ADTO_W<GOTGINTF_SPEC, 18> {
        ADTO_W::new(self)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    #[must_use]
    pub fn df(&mut self) -> DF_W<GOTGINTF_SPEC, 19> {
        DF_W::new(self)
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
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgintf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgintf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINTF_SPEC;
impl crate::RegisterSpec for GOTGINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgintf::R`](R) reader structure"]
impl crate::Readable for GOTGINTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgintf::W`](W) writer structure"]
impl crate::Writable for GOTGINTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGINTF to value 0"]
impl crate::Resettable for GOTGINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
