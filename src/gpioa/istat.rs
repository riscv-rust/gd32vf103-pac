#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<ISTAT_SPEC>;
#[doc = "Field `ISTAT0` reader - Port input status"]
pub type ISTAT0_R = crate::BitReader;
#[doc = "Field `ISTAT1` reader - Port input status"]
pub type ISTAT1_R = crate::BitReader;
#[doc = "Field `ISTAT2` reader - Port input status"]
pub type ISTAT2_R = crate::BitReader;
#[doc = "Field `ISTAT3` reader - Port input status"]
pub type ISTAT3_R = crate::BitReader;
#[doc = "Field `ISTAT4` reader - Port input status"]
pub type ISTAT4_R = crate::BitReader;
#[doc = "Field `ISTAT5` reader - Port input status"]
pub type ISTAT5_R = crate::BitReader;
#[doc = "Field `ISTAT6` reader - Port input status"]
pub type ISTAT6_R = crate::BitReader;
#[doc = "Field `ISTAT7` reader - Port input status"]
pub type ISTAT7_R = crate::BitReader;
#[doc = "Field `ISTAT8` reader - Port input status"]
pub type ISTAT8_R = crate::BitReader;
#[doc = "Field `ISTAT9` reader - Port input status"]
pub type ISTAT9_R = crate::BitReader;
#[doc = "Field `ISTAT10` reader - Port input status"]
pub type ISTAT10_R = crate::BitReader;
#[doc = "Field `ISTAT11` reader - Port input status"]
pub type ISTAT11_R = crate::BitReader;
#[doc = "Field `ISTAT12` reader - Port input status"]
pub type ISTAT12_R = crate::BitReader;
#[doc = "Field `ISTAT13` reader - Port input status"]
pub type ISTAT13_R = crate::BitReader;
#[doc = "Field `ISTAT14` reader - Port input status"]
pub type ISTAT14_R = crate::BitReader;
#[doc = "Field `ISTAT15` reader - Port input status"]
pub type ISTAT15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input status"]
    #[inline(always)]
    pub fn istat0(&self) -> ISTAT0_R {
        ISTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input status"]
    #[inline(always)]
    pub fn istat1(&self) -> ISTAT1_R {
        ISTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input status"]
    #[inline(always)]
    pub fn istat2(&self) -> ISTAT2_R {
        ISTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input status"]
    #[inline(always)]
    pub fn istat3(&self) -> ISTAT3_R {
        ISTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input status"]
    #[inline(always)]
    pub fn istat4(&self) -> ISTAT4_R {
        ISTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input status"]
    #[inline(always)]
    pub fn istat5(&self) -> ISTAT5_R {
        ISTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input status"]
    #[inline(always)]
    pub fn istat6(&self) -> ISTAT6_R {
        ISTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input status"]
    #[inline(always)]
    pub fn istat7(&self) -> ISTAT7_R {
        ISTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input status"]
    #[inline(always)]
    pub fn istat8(&self) -> ISTAT8_R {
        ISTAT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input status"]
    #[inline(always)]
    pub fn istat9(&self) -> ISTAT9_R {
        ISTAT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input status"]
    #[inline(always)]
    pub fn istat10(&self) -> ISTAT10_R {
        ISTAT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input status"]
    #[inline(always)]
    pub fn istat11(&self) -> ISTAT11_R {
        ISTAT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input status"]
    #[inline(always)]
    pub fn istat12(&self) -> ISTAT12_R {
        ISTAT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input status"]
    #[inline(always)]
    pub fn istat13(&self) -> ISTAT13_R {
        ISTAT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input status"]
    #[inline(always)]
    pub fn istat14(&self) -> ISTAT14_R {
        ISTAT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input status"]
    #[inline(always)]
    pub fn istat15(&self) -> ISTAT15_R {
        ISTAT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port input status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
