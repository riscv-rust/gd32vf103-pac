#[doc = "Register `RFIFOMDATA10` reader"]
pub struct R(crate::R<RFIFOMDATA10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOMDATA10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOMDATA10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOMDATA10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DB4` reader - Data byte 4"]
pub type DB4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB5` reader - Data byte 5"]
pub type DB5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB6` reader - Data byte 6"]
pub type DB6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB7` reader - Data byte 7"]
pub type DB7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO0 mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata10](index.html) module"]
pub struct RFIFOMDATA10_SPEC;
impl crate::RegisterSpec for RFIFOMDATA10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifomdata10::R](R) reader structure"]
impl crate::Readable for RFIFOMDATA10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFOMDATA10 to value 0"]
impl crate::Resettable for RFIFOMDATA10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
