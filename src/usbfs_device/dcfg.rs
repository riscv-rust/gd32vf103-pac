#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DS` reader - Device speed"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - Device speed"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOH` reader - Non-zero-length status OUT handshake"]
pub type NZLSOH_R = crate::BitReader;
#[doc = "Field `NZLSOH` writer - Non-zero-length status OUT handshake"]
pub type NZLSOH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Device address"]
pub type DAR_R = crate::FieldReader;
#[doc = "Field `DAR` writer - Device address"]
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EOPFT` reader - end of periodic frame time"]
pub type EOPFT_R = crate::FieldReader;
#[doc = "Field `EOPFT` writer - end of periodic frame time"]
pub type EOPFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&self) -> NZLSOH_R {
        NZLSOH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&self) -> EOPFT_R {
        EOPFT_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<DCFG_SPEC> {
        DS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzlsoh(&mut self) -> NZLSOH_W<DCFG_SPEC> {
        NZLSOH_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<DCFG_SPEC> {
        DAR_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    #[must_use]
    pub fn eopft(&mut self) -> EOPFT_W<DCFG_SPEC> {
        EOPFT_W::new(self, 11)
    }
}
#[doc = "device configuration register (DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG to value 0"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
