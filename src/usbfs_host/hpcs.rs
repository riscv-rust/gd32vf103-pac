#[doc = "Register `HPCS` reader"]
pub type R = crate::R<HPCS_SPEC>;
#[doc = "Register `HPCS` writer"]
pub type W = crate::W<HPCS_SPEC>;
#[doc = "Field `PCST` reader - Port connect status"]
pub type PCST_R = crate::BitReader;
#[doc = "Field `PCD` reader - Port connect detected"]
pub type PCD_R = crate::BitReader;
#[doc = "Field `PCD` writer - Port connect detected"]
pub type PCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Port enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Port enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDC` reader - Port enable/disable change"]
pub type PEDC_R = crate::BitReader;
#[doc = "Field `PEDC` writer - Port enable/disable change"]
pub type PEDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREM` reader - Port resume"]
pub type PREM_R = crate::BitReader;
#[doc = "Field `PREM` writer - Port resume"]
pub type PREM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSP` reader - Port suspend"]
pub type PSP_R = crate::BitReader;
#[doc = "Field `PSP` writer - Port suspend"]
pub type PSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PRST_R = crate::BitReader;
#[doc = "Field `PRST` writer - Port reset"]
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLST` reader - Port line status"]
pub type PLST_R = crate::FieldReader;
#[doc = "Field `PP` reader - Port power"]
pub type PP_R = crate::BitReader;
#[doc = "Field `PP` writer - Port power"]
pub type PP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn pcd(&mut self) -> PCD_W<HPCS_SPEC> {
        PCD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<HPCS_SPEC> {
        PE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn pedc(&mut self) -> PEDC_W<HPCS_SPEC> {
        PEDC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn prem(&mut self) -> PREM_W<HPCS_SPEC> {
        PREM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn psp(&mut self) -> PSP_W<HPCS_SPEC> {
        PSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<HPCS_SPEC> {
        PRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<HPCS_SPEC> {
        PP_W::new(self, 12)
    }
}
#[doc = "Host port control and status register (USBFS_HPCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCS_SPEC;
impl crate::RegisterSpec for HPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcs::R`](R) reader structure"]
impl crate::Readable for HPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcs::W`](W) writer structure"]
impl crate::Writable for HPCS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPCS to value 0"]
impl crate::Resettable for HPCS_SPEC {
    const RESET_VALUE: u32 = 0;
}
