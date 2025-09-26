#[doc = "Register `RFIFOMDATA11` reader"]
pub type R = crate::R<RFIFOMDATA11_SPEC>;
#[doc = "Field `DB4` reader - Data byte 4"]
pub type DB4_R = crate::FieldReader;
#[doc = "Field `DB5` reader - Data byte 5"]
pub type DB5_R = crate::FieldReader;
#[doc = "Field `DB6` reader - Data byte 6"]
pub type DB6_R = crate::FieldReader;
#[doc = "Field `DB7` reader - Data byte 7"]
pub type DB7_R = crate::FieldReader;
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
#[doc = "Receive FIFO1 mailbox data1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifomdata11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFOMDATA11_SPEC;
impl crate::RegisterSpec for RFIFOMDATA11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifomdata11::R`](R) reader structure"]
impl crate::Readable for RFIFOMDATA11_SPEC {}
#[doc = "`reset()` method sets RFIFOMDATA11 to value 0"]
impl crate::Resettable for RFIFOMDATA11_SPEC {
    const RESET_VALUE: u32 = 0;
}
