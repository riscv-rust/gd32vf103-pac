#[doc = "Register `BOP` writer"]
pub type W = crate::W<BOP_SPEC>;
#[doc = "Field `BOP0` writer - Port 0 Set bit"]
pub type BOP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP1` writer - Port 1 Set bit"]
pub type BOP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP2` writer - Port 2 Set bit"]
pub type BOP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP3` writer - Port 3 Set bit"]
pub type BOP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP4` writer - Port 4 Set bit"]
pub type BOP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP5` writer - Port 5 Set bit"]
pub type BOP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP6` writer - Port 6 Set bit"]
pub type BOP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP7` writer - Port 7 Set bit"]
pub type BOP7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP8` writer - Port 8 Set bit"]
pub type BOP8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP9` writer - Port 9 Set bit"]
pub type BOP9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP10` writer - Port 10 Set bit"]
pub type BOP10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP11` writer - Port 11 Set bit"]
pub type BOP11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP12` writer - Port 12 Set bit"]
pub type BOP12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP13` writer - Port 13 Set bit"]
pub type BOP13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP14` writer - Port 14 Set bit"]
pub type BOP14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOP15` writer - Port 15 Set bit"]
pub type BOP15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR0` writer - Port 0 Clear bit"]
pub type CR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR1` writer - Port 1 Clear bit"]
pub type CR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR2` writer - Port 2 Clear bit"]
pub type CR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR3` writer - Port 3 Clear bit"]
pub type CR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR4` writer - Port 4 Clear bit"]
pub type CR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR5` writer - Port 5 Clear bit"]
pub type CR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR6` writer - Port 6 Clear bit"]
pub type CR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR7` writer - Port 7 Clear bit"]
pub type CR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR8` writer - Port 8 Clear bit"]
pub type CR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR9` writer - Port 9 Clear bit"]
pub type CR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR10` writer - Port 10 Clear bit"]
pub type CR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR11` writer - Port 11 Clear bit"]
pub type CR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR12` writer - Port 12 Clear bit"]
pub type CR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR13` writer - Port 13 Clear bit"]
pub type CR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR14` writer - Port 14 Clear bit"]
pub type CR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR15` writer - Port 15 Clear bit"]
pub type CR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port 0 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop0(&mut self) -> BOP0_W<BOP_SPEC> {
        BOP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop1(&mut self) -> BOP1_W<BOP_SPEC> {
        BOP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop2(&mut self) -> BOP2_W<BOP_SPEC> {
        BOP2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop3(&mut self) -> BOP3_W<BOP_SPEC> {
        BOP3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop4(&mut self) -> BOP4_W<BOP_SPEC> {
        BOP4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop5(&mut self) -> BOP5_W<BOP_SPEC> {
        BOP5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop6(&mut self) -> BOP6_W<BOP_SPEC> {
        BOP6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop7(&mut self) -> BOP7_W<BOP_SPEC> {
        BOP7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop8(&mut self) -> BOP8_W<BOP_SPEC> {
        BOP8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop9(&mut self) -> BOP9_W<BOP_SPEC> {
        BOP9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop10(&mut self) -> BOP10_W<BOP_SPEC> {
        BOP10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop11(&mut self) -> BOP11_W<BOP_SPEC> {
        BOP11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop12(&mut self) -> BOP12_W<BOP_SPEC> {
        BOP12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop13(&mut self) -> BOP13_W<BOP_SPEC> {
        BOP13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop14(&mut self) -> BOP14_W<BOP_SPEC> {
        BOP14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 Set bit"]
    #[inline(always)]
    #[must_use]
    pub fn bop15(&mut self) -> BOP15_W<BOP_SPEC> {
        BOP15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<BOP_SPEC> {
        CR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<BOP_SPEC> {
        CR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<BOP_SPEC> {
        CR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<BOP_SPEC> {
        CR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<BOP_SPEC> {
        CR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<BOP_SPEC> {
        CR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<BOP_SPEC> {
        CR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<BOP_SPEC> {
        CR7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<BOP_SPEC> {
        CR8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<BOP_SPEC> {
        CR9_W::new(self, 25)
    }
    #[doc = "Bit 26 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<BOP_SPEC> {
        CR10_W::new(self, 26)
    }
    #[doc = "Bit 27 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<BOP_SPEC> {
        CR11_W::new(self, 27)
    }
    #[doc = "Bit 28 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<BOP_SPEC> {
        CR12_W::new(self, 28)
    }
    #[doc = "Bit 29 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<BOP_SPEC> {
        CR13_W::new(self, 29)
    }
    #[doc = "Bit 30 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<BOP_SPEC> {
        CR14_W::new(self, 30)
    }
    #[doc = "Bit 31 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<BOP_SPEC> {
        CR15_W::new(self, 31)
    }
}
#[doc = "Port bit operate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOP_SPEC;
impl crate::RegisterSpec for BOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bop::W`](W) writer structure"]
impl crate::Writable for BOP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOP to value 0"]
impl crate::Resettable for BOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
