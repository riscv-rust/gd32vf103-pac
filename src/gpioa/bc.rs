#[doc = "Register `BC` writer"]
pub struct W(crate::W<BC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR0` writer - Port 0 Clear bit"]
pub type CR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR1` writer - Port 1 Clear bit"]
pub type CR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR2` writer - Port 2 Clear bit"]
pub type CR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR3` writer - Port 3 Clear bit"]
pub type CR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR4` writer - Port 4 Clear bit"]
pub type CR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR5` writer - Port 5 Clear bit"]
pub type CR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR6` writer - Port 6 Clear bit"]
pub type CR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR7` writer - Port 7 Clear bit"]
pub type CR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR8` writer - Port 8 Clear bit"]
pub type CR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR9` writer - Port 9 Clear bit"]
pub type CR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR10` writer - Port 10 Clear bit"]
pub type CR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR11` writer - Port 11 Clear bit"]
pub type CR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR12` writer - Port 12 Clear bit"]
pub type CR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR13` writer - Port 13 Clear bit"]
pub type CR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR14` writer - Port 14 Clear bit"]
pub type CR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
#[doc = "Field `CR15` writer - Port 15 Clear bit"]
pub type CR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, BC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Port 0 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr0(&mut self) -> CR0_W<0> {
        CR0_W::new(self)
    }
    #[doc = "Bit 1 - Port 1 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr1(&mut self) -> CR1_W<1> {
        CR1_W::new(self)
    }
    #[doc = "Bit 2 - Port 2 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr2(&mut self) -> CR2_W<2> {
        CR2_W::new(self)
    }
    #[doc = "Bit 3 - Port 3 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr3(&mut self) -> CR3_W<3> {
        CR3_W::new(self)
    }
    #[doc = "Bit 4 - Port 4 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr4(&mut self) -> CR4_W<4> {
        CR4_W::new(self)
    }
    #[doc = "Bit 5 - Port 5 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr5(&mut self) -> CR5_W<5> {
        CR5_W::new(self)
    }
    #[doc = "Bit 6 - Port 6 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr6(&mut self) -> CR6_W<6> {
        CR6_W::new(self)
    }
    #[doc = "Bit 7 - Port 7 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr7(&mut self) -> CR7_W<7> {
        CR7_W::new(self)
    }
    #[doc = "Bit 8 - Port 8 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr8(&mut self) -> CR8_W<8> {
        CR8_W::new(self)
    }
    #[doc = "Bit 9 - Port 9 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr9(&mut self) -> CR9_W<9> {
        CR9_W::new(self)
    }
    #[doc = "Bit 10 - Port 10 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr10(&mut self) -> CR10_W<10> {
        CR10_W::new(self)
    }
    #[doc = "Bit 11 - Port 11 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr11(&mut self) -> CR11_W<11> {
        CR11_W::new(self)
    }
    #[doc = "Bit 12 - Port 12 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr12(&mut self) -> CR12_W<12> {
        CR12_W::new(self)
    }
    #[doc = "Bit 13 - Port 13 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr13(&mut self) -> CR13_W<13> {
        CR13_W::new(self)
    }
    #[doc = "Bit 14 - Port 14 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr14(&mut self) -> CR14_W<14> {
        CR14_W::new(self)
    }
    #[doc = "Bit 15 - Port 15 Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cr15(&mut self) -> CR15_W<15> {
        CR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bc](index.html) module"]
pub struct BC_SPEC;
impl crate::RegisterSpec for BC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bc::W](W) writer structure"]
impl crate::Writable for BC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BC to value 0"]
impl crate::Resettable for BC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
