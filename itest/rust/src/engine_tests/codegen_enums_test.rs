/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Tests the presence, naming and accessibility of generated enum and enumerator symbols.
// Only enabled in full codegen mode (including experimental APIs).

#![cfg(feature = "codegen-full-experimental")]

use crate::framework::itest;

use godot::engine::audio_effect_spectrum_analyzer::FftSize;
use godot::engine::base_material_3d::Flags;
use godot::engine::camera_2d::Camera2DProcessCallback;
use godot::engine::camera_3d::ProjectionType;
use godot::engine::cpu_particles_2d::{Parameter, ParticleFlags};
use godot::engine::editor_plugin::CustomControlContainer;
use godot::engine::environment::SdfgiYScale;
use godot::engine::file_access::{CompressionMode, ModeFlags};
use godot::engine::http_client::ResponseCode;
use godot::engine::image::Format;
use godot::engine::mesh::ArrayType;
use godot::engine::navigation_path_query_parameters_2d::PathPostProcessing;
use godot::engine::node::ProcessMode;
use godot::engine::object::ConnectFlags;
use godot::engine::open_xr_action::ActionType;
use godot::engine::open_xr_hand::Hands;
use godot::engine::open_xr_interface::HandJointFlags;
use godot::engine::os::SystemDir;
use godot::engine::physical_bone_3d::JointType;
use godot::engine::physics_server_2d::{AreaParameter, BodyMode, CcdMode};
use godot::engine::physics_server_3d::{
    AreaSpaceOverrideMode, G6dofJointAxisParam, ProcessInfo, SpaceParameter,
};
use godot::engine::rendering_device::{
    CompareOperator, PipelineDynamicStateFlags, StencilOperation,
};
use godot::engine::rendering_server::{
    ArrayFormat, CubeMapLayer, EnvironmentSdfgiYScale, EnvironmentSsaoQuality, Features,
    GlobalShaderParameterType, MultimeshTransformFormat, RenderingInfo, ViewportScaling3DMode,
    VoxelGiQuality,
};
use godot::engine::resource_format_loader::CacheMode;
use godot::engine::resource_loader::ThreadLoadStatus;
use godot::engine::rigid_body_2d::CenterOfMassMode;
use godot::engine::scene_state::GenEditState;
use godot::engine::shader::Mode;
use godot::engine::sub_viewport::UpdateMode;
use godot::engine::time::Month;
use godot::engine::upnp::UpnpResult;
use godot::engine::viewport::Msaa;
use godot::engine::visual_shader_node_float_op::Operator;
use godot::engine::visual_shader_node_vector_func::Function;
use godot::engine::voxel_gi::Subdiv;
use godot::engine::xr_interface::{EnvironmentBlendMode, TrackingStatus};
use godot::engine::xr_pose::TrackingConfidence;
use godot::engine::zip_packer::ZipAppend;

#[itest]
fn codegen_enums_exist() {
    // Remove entire enum name.
    let _ = ModeFlags::ReadWrite;
    let _ = BodyMode::Kinematic;
    let _ = CacheMode::Ignore;
    let _ = CenterOfMassMode::Auto;
    let _ = Format::Rf;
    let _ = GenEditState::Disabled;
    let _ = JointType::Pin;
    let _ = Mode::Sky;
    let _ = Month::February;
    let _ = ProcessMode::WhenPaused;
    let _ = RenderingInfo::BufferMemUsed;
    let _ = SystemDir::Dcim;

    // Remove entire name, but MiXED case.
    let _ = VoxelGiQuality::Low;
    let _ = CcdMode::CastRay;
    let _ = UpnpResult::HttpError;
    let _ = SdfgiYScale::Scale100Percent;
    let _ = EnvironmentSdfgiYScale::Scale50Percent;

    // Entire enum name, but changed.
    let _ = Parameter::InitialLinearVelocity;
    let _ = SpaceParameter::ContactMaxSeparation;
    let _ = AreaParameter::Gravity;
    let _ = StencilOperation::Keep;
    let _ = CompareOperator::Less;
    let _ = CubeMapLayer::Right;
    let _ = Camera2DProcessCallback::Physics;

    // Prefix omitted.
    let _ = ArrayType::Custom0;
    let _ = PathPostProcessing::Edgecentered; // only in experimental API
    let _ = PipelineDynamicStateFlags::DepthBias;
    let _ = ProcessInfo::CollisionPairs;
    let _ = ResponseCode::NoContent;
    let _ = UpdateMode::WhenVisible;
    let _ = ZipAppend::Create;

    // Plural.
    let _ = Hands::Left;
    let _ = Features::Shaders;
    let _ = Flags::AlbedoTextureForceSrgb;

    // Unrelated name.
    let _ = GlobalShaderParameterType::Bool;
    let _ = ArrayFormat::FlagFormatVersion2;
    let _ = CustomControlContainer::CanvasEditorSideLeft;

    // Implicitly used class name instead of enum name (OpenXR*, XR*).
    let _ = ActionType::Pose;
    let _ = TrackingConfidence::None;
    let _ = TrackingStatus::NotTracking;
    let _ = EnvironmentBlendMode::Opaque;

    // Abbreviation.
    let _ = Operator::Atan2;
    let _ = Function::Log;
    let _ = EnvironmentSsaoQuality::High;

    // Remove postfix (Mode, Type, Flags, Param, ...).
    let _ = CompressionMode::Deflate;
    let _ = AreaSpaceOverrideMode::Combine;
    let _ = ProjectionType::Orthogonal;
    let _ = ConnectFlags::Persist;
    let _ = HandJointFlags::OrientationTracked;
    let _ = ParticleFlags::RotateY;
    let _ = G6dofJointAxisParam::LinearLowerLimit;
    let _ = ThreadLoadStatus::InvalidResource;
    let _ = ViewportScaling3DMode::Bilinear;

    // Remaining identifier is non-valid (digit).
    let _ = Subdiv::Subdiv64;
    let _ = FftSize::Size256;
    let _ = Msaa::Msaa8x;
    let _ = MultimeshTransformFormat::Transform3d;
}
