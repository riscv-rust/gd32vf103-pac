#[doc = "Register `HPCS` reader"]
pub struct R(crate::R<HPCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPCS` writer"]
pub struct W(crate::W<HPCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPCS_SPEC>;
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
impl From<crate::W<HPCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCST` reader - Port connect status"]
pub type PCST_R = crate::BitReader<bool>;
#[doc = "Field `PCD` reader - Port connect detected"]
pub type PCD_R = crate::BitReader<bool>;
#[doc = "Field `PCD` writer - Port connect detected"]
pub type PCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PE` reader - Port enable"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Port enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PEDC` reader - Port enable/disable change"]
pub type PEDC_R = crate::BitReader<bool>;
#[doc = "Field `PEDC` writer - Port enable/disable change"]
pub type PEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PREM` reader - Port resume"]
pub type PREM_R = crate::BitReader<bool>;
#[doc = "Field `PREM` writer - Port resume"]
pub type PREM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PSP` reader - Port suspend"]
pub type PSP_R = crate::BitReader<bool>;
#[doc = "Field `PSP` writer - Port suspend"]
pub type PSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PRST_R = crate::BitReader<bool>;
#[doc = "Field `PRST` writer - Port reset"]
pub type PRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PLST` reader - Port line status"]
pub type PLST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PP` reader - Port power"]
pub type PP_R = crate::BitReader<bool>;
#[doc = "Field `PP` writer - Port power"]
pub type PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCS_SPEC, bool, O>;
#[doc = "Field `PS` reader - Port speed"]
pub type PS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn pcst(&self) -> PCST_R {
        PCST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn prem(&self) -> PREM_R {
        PREM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psp(&self) -> PSP_R {
        PSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn plst(&self) -> PLST_R {
        PLST_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    #[must_use]
    pub fn pcd(&mut self) -> PCD_W<1> {
        PCD_W::new(self)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<2> {
        PE_W::new(self)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn pedc(&mut self) -> PEDC_W<3> {
        PEDC_W::new(self)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn prem(&mut self) -> PREM_W<6> {
        PREM_W::new(self)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn psp(&mut self) -> PSP_W<7> {
        PSP_W::new(self)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<8> {
        PRST_W::new(self)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<12> {
        PP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host port control and status register (USBFS_HPCS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcs](index.html) module"]
pub struct HPCS_SPEC;
impl crate::RegisterSpec for HPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpcs::R](R) reader structure"]
impl crate::Readable for HPCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpcs::W](W) writer structure"]
impl crate::Writable for HPCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPCS to value 0"]
impl crate::Resettable for HPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
