#[doc = "Register `HPCS` reader"]
pub type R = crate::R<HPCS_SPEC>;
#[doc = "Register `HPCS` writer"]
pub type W = crate::W<HPCS_SPEC>;
#[doc = "Field `PCST` reader - Port connect status"]
pub type PCST_R = crate::BitReader;
#[doc = "Field `PCD` reader - Port connect detected"]
pub type PCD_R = crate::BitReader;
#[doc = "Field `PCD` writer - Port connect detected"]
pub type PCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE` reader - Port enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Port enable"]
pub type PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEDC` reader - Port enable/disable change"]
pub type PEDC_R = crate::BitReader;
#[doc = "Field `PEDC` writer - Port enable/disable change"]
pub type PEDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PREM` reader - Port resume"]
pub type PREM_R = crate::BitReader;
#[doc = "Field `PREM` writer - Port resume"]
pub type PREM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSP` reader - Port suspend"]
pub type PSP_R = crate::BitReader;
#[doc = "Field `PSP` writer - Port suspend"]
pub type PSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PRST_R = crate::BitReader;
#[doc = "Field `PRST` writer - Port reset"]
pub type PRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLST` reader - Port line status"]
pub type PLST_R = crate::FieldReader;
#[doc = "Field `PP` reader - Port power"]
pub type PP_R = crate::BitReader;
#[doc = "Field `PP` writer - Port power"]
pub type PP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PS` reader - Port speed"]
pub type PS_R = crate::FieldReader;
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
    pub fn pcd(&mut self) -> PCD_W<HPCS_SPEC, 1> {
        PCD_W::new(self)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<HPCS_SPEC, 2> {
        PE_W::new(self)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn pedc(&mut self) -> PEDC_W<HPCS_SPEC, 3> {
        PEDC_W::new(self)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn prem(&mut self) -> PREM_W<HPCS_SPEC, 6> {
        PREM_W::new(self)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn psp(&mut self) -> PSP_W<HPCS_SPEC, 7> {
        PSP_W::new(self)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<HPCS_SPEC, 8> {
        PRST_W::new(self)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<HPCS_SPEC, 12> {
        PP_W::new(self)
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
#[doc = "Host port control and status register (USBFS_HPCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCS_SPEC;
impl crate::RegisterSpec for HPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcs::R`](R) reader structure"]
impl crate::Readable for HPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcs::W`](W) writer structure"]
impl crate::Writable for HPCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPCS to value 0"]
impl crate::Resettable for HPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
