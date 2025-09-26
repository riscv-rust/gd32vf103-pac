#[doc = "Register `SNTCFG0` reader"]
pub type R = crate::R<SNTCFG0_SPEC>;
#[doc = "Register `SNTCFG0` writer"]
pub type W = crate::W<SNTCFG0_SPEC>;
#[doc = "Field `ASET` reader - Address setup time"]
pub type ASET_R = crate::FieldReader;
#[doc = "Field `ASET` writer - Address setup time"]
pub type ASET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHLD` reader - Address hold time"]
pub type AHLD_R = crate::FieldReader;
#[doc = "Field `AHLD` writer - Address hold time"]
pub type AHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSET` reader - Data setup time"]
pub type DSET_R = crate::FieldReader;
#[doc = "Field `DSET` writer - Data setup time"]
pub type DSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AHLD_R {
        AHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DSET_R {
        DSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> ASET_W<SNTCFG0_SPEC> {
        ASET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ahld(&mut self) -> AHLD_W<SNTCFG0_SPEC> {
        AHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dset(&mut self) -> DSET_W<SNTCFG0_SPEC> {
        DSET_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BUSLAT_W<SNTCFG0_SPEC> {
        BUSLAT_W::new(self, 16)
    }
}
#[doc = "SRAM/NOR flash timing configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sntcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sntcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNTCFG0_SPEC;
impl crate::RegisterSpec for SNTCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sntcfg0::R`](R) reader structure"]
impl crate::Readable for SNTCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sntcfg0::W`](W) writer structure"]
impl crate::Writable for SNTCFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNTCFG0 to value 0x0fff_ffff"]
impl crate::Resettable for SNTCFG0_SPEC {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
