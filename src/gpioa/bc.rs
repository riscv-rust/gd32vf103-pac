#[doc = "Register `BC` writer"]
pub type W = crate::W<BC_SPEC>;
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
    #[doc = "Bit 0 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<BC_SPEC> {
        CR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<BC_SPEC> {
        CR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<BC_SPEC> {
        CR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<BC_SPEC> {
        CR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<BC_SPEC> {
        CR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<BC_SPEC> {
        CR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<BC_SPEC> {
        CR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<BC_SPEC> {
        CR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<BC_SPEC> {
        CR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<BC_SPEC> {
        CR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<BC_SPEC> {
        CR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<BC_SPEC> {
        CR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<BC_SPEC> {
        CR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<BC_SPEC> {
        CR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<BC_SPEC> {
        CR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<BC_SPEC> {
        CR15_W::new(self, 15)
    }
}
#[doc = "Port bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BC_SPEC;
impl crate::RegisterSpec for BC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bc::W`](W) writer structure"]
impl crate::Writable for BC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BC to value 0"]
impl crate::Resettable for BC_SPEC {
    const RESET_VALUE: u32 = 0;
}
