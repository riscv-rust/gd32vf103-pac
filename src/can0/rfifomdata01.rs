#[doc = "Register `RFIFOMDATA01` reader"]
pub struct R(crate::R<RFIFOMDATA01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOMDATA01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOMDATA01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOMDATA01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DB0` reader - Data byte 0"]
pub type DB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB1` reader - Data byte 1"]
pub type DB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB2` reader - Data byte 2"]
pub type DB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB3` reader - Data byte 3"]
pub type DB3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO1 mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata01](index.html) module"]
pub struct RFIFOMDATA01_SPEC;
impl crate::RegisterSpec for RFIFOMDATA01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifomdata01::R](R) reader structure"]
impl crate::Readable for RFIFOMDATA01_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFOMDATA01 to value 0"]
impl crate::Resettable for RFIFOMDATA01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
