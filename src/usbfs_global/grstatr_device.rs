#[doc = "Register `GRSTATR_Device` reader"]
pub type R = crate::R<GRSTATR_DEVICE_SPEC>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `BCOUNT` reader - Byte count"]
pub type BCOUNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `RPCKST` reader - Recieve packet status"]
pub type RPCKST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcount(&self) -> BCOUNT_R {
        BCOUNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Recieve packet status"]
    #[inline(always)]
    pub fn rpckst(&self) -> RPCKST_R {
        RPCKST_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "Global Receive status read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstatr_device::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTATR_DEVICE_SPEC;
impl crate::RegisterSpec for GRSTATR_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstatr_device::R`](R) reader structure"]
impl crate::Readable for GRSTATR_DEVICE_SPEC {}
#[doc = "`reset()` method sets GRSTATR_Device to value 0"]
impl crate::Resettable for GRSTATR_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
