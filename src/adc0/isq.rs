#[doc = "Register `ISQ` reader"]
pub type R = crate::R<ISQ_SPEC>;
#[doc = "Register `ISQ` writer"]
pub type W = crate::W<ISQ_SPEC>;
#[doc = "Field `ISQ0` reader - 1st conversion in inserted sequence"]
pub type ISQ0_R = crate::FieldReader;
#[doc = "Field `ISQ0` writer - 1st conversion in inserted sequence"]
pub type ISQ0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ISQ1` reader - 2nd conversion in inserted sequence"]
pub type ISQ1_R = crate::FieldReader;
#[doc = "Field `ISQ1` writer - 2nd conversion in inserted sequence"]
pub type ISQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ISQ2` reader - 3rd conversion in inserted sequence"]
pub type ISQ2_R = crate::FieldReader;
#[doc = "Field `ISQ2` writer - 3rd conversion in inserted sequence"]
pub type ISQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ISQ3` reader - 4th conversion in inserted sequence"]
pub type ISQ3_R = crate::FieldReader;
#[doc = "Field `ISQ3` writer - 4th conversion in inserted sequence"]
pub type ISQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `IL` reader - Inserted channel group length"]
pub type IL_R = crate::FieldReader;
#[doc = "Field `IL` writer - Inserted channel group length"]
pub type IL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq0(&self) -> ISQ0_R {
        ISQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq1(&self) -> ISQ1_R {
        ISQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq2(&self) -> ISQ2_R {
        ISQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq3(&self) -> ISQ3_R {
        ISQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    pub fn il(&self) -> IL_R {
        IL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq0(&mut self) -> ISQ0_W<ISQ_SPEC, 0> {
        ISQ0_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq1(&mut self) -> ISQ1_W<ISQ_SPEC, 5> {
        ISQ1_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq2(&mut self) -> ISQ2_W<ISQ_SPEC, 10> {
        ISQ2_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn isq3(&mut self) -> ISQ3_W<ISQ_SPEC, 15> {
        ISQ3_W::new(self)
    }
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    #[must_use]
    pub fn il(&mut self) -> IL_W<ISQ_SPEC, 20> {
        IL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Inserted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISQ_SPEC;
impl crate::RegisterSpec for ISQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isq::R`](R) reader structure"]
impl crate::Readable for ISQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isq::W`](W) writer structure"]
impl crate::Writable for ISQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISQ to value 0"]
impl crate::Resettable for ISQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
