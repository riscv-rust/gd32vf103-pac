#[doc = "Register `OBSTAT` reader"]
pub type R = crate::R<OBSTAT_SPEC>;
#[doc = "Field `OBERR` reader - Option bytes read error bit"]
pub type OBERR_R = crate::BitReader;
#[doc = "Field `SPC` reader - Option bytes security protection code"]
pub type SPC_R = crate::BitReader;
#[doc = "Field `USER` reader - Store USER of option bytes block after system reset"]
pub type USER_R = crate::FieldReader;
#[doc = "Field `DATA` reader - Store DATA\\[15:0\\]
of option bytes block after system reset"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Option bytes read error bit"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option bytes security protection code"]
    #[inline(always)]
    pub fn spc(&self) -> SPC_R {
        SPC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Store USER of option bytes block after system reset"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:25 - Store DATA\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
}
#[doc = "Option byte status register\n\nYou can [`read`](crate::Reg::read) this register and get [`obstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBSTAT_SPEC;
impl crate::RegisterSpec for OBSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obstat::R`](R) reader structure"]
impl crate::Readable for OBSTAT_SPEC {}
#[doc = "`reset()` method sets OBSTAT to value 0"]
impl crate::Resettable for OBSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
