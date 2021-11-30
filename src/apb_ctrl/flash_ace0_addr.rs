#[doc = "Register `FLASH_ACE0_ADDR` reader"]
pub struct R(crate::R<FLASH_ACE0_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACE0_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACE0_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACE0_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ACE0_ADDR` writer"]
pub struct W(crate::W<FLASH_ACE0_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACE0_ADDR_SPEC>;
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
impl From<crate::W<FLASH_ACE0_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACE0_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S` reader - reg_flash_ace0_addr_s"]
pub struct S_R(crate::FieldReader<u32, u32>);
impl S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` writer - reg_flash_ace0_addr_s"]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reg_flash_ace0_addr_s"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_flash_ace0_addr_s"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_FLASH_ACE0_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace0_addr](index.html) module"]
pub struct FLASH_ACE0_ADDR_SPEC;
impl crate::RegisterSpec for FLASH_ACE0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ace0_addr::R](R) reader structure"]
impl crate::Readable for FLASH_ACE0_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ace0_addr::W](W) writer structure"]
impl crate::Writable for FLASH_ACE0_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ACE0_ADDR to value 0"]
impl crate::Resettable for FLASH_ACE0_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
