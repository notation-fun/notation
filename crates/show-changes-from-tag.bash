#!/usr/bin/env bash

cd `dirname $0`

echo "------------------------------------- core"
git shortlog --numbered --summary $1..HEAD -- notation_core
echo "------------------------------------- fretted"
git shortlog --numbered --summary $1..HEAD -- notation_fretted
echo "------------------------------------- guitar"
git shortlog --numbered --summary $1..HEAD -- notation_guitar
echo "------------------------------------- proto"
git shortlog --numbered --summary $1..HEAD -- notation_proto
echo "------------------------------------- model"
git shortlog --numbered --summary $1..HEAD -- notation_model
echo "------------------------------------- dsl"
git shortlog --numbered --summary $1..HEAD -- notation_dsl
echo "------------------------------------- macro"
git shortlog --numbered --summary $1..HEAD -- notation_macro
echo "------------------------------------- tab"
git shortlog --numbered --summary $1..HEAD -- notation_tab
echo "------------------------------------- audio"
git shortlog --numbered --summary $1..HEAD -- notation_audio
echo "------------------------------------- midi"
git shortlog --numbered --summary $1..HEAD -- notation_midi
echo "------------------------------------- bevy_utils"
git shortlog --numbered --summary $1..HEAD -- notation_bevy_utils
echo "------------------------------------- bevy"
git shortlog --numbered --summary $1..HEAD -- notation_bevy
