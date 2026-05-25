#  SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com> */
#  SPDX-License-Identifier: GPL-3.0-or-later */

#    Copyright (C) 2026 Ahum Maitra

#       This program is free software: you can redistribute it and/or modify
#       it under the terms of the GNU General Public License as published by
#       the Free Software Foundation, either version 3 of the License, or
#       (at your option) any later version.

#       This program is distributed in the hope that it will be useful,
#       but WITHOUT ANY WARRANTY; without even the implied warranty of
#       MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#       GNU General Public License for more details.

#       You should have received a copy of the GNU General Public License
#       along with this program.  If not, see <https://www.gnu.org/licenses/>. 


function fish_prompt --description 'Write out the prompt'
        set -l last_status $status
    
        prompt_login
    
        echo -n ':'
    
        # PWD
        set_color $fish_color_cwd
        echo -n (prompt_pwd)
        set_color --reset
    
        set -q __fish_git_prompt_showdirtystate
        or set -g __fish_git_prompt_showdirtystate 1
        set -q __fish_git_prompt_showuntrackedfiles
        or set -g __fish_git_prompt_showuntrackedfiles 1
        set -q __fish_git_prompt_showcolorhints
        or set -g __fish_git_prompt_showcolorhints 1
        set -q __fish_git_prompt_color_untrackedfiles
        or set -g __fish_git_prompt_color_untrackedfiles yellow
        set -q __fish_git_prompt_char_untrackedfiles
        or set -g __fish_git_prompt_char_untrackedfiles '?'
        set -q __fish_git_prompt_color_invalidstate
        or set -g __fish_git_prompt_color_invalidstate red
        set -q __fish_git_prompt_char_invalidstate
        or set -g __fish_git_prompt_char_invalidstate '!'
        set -q __fish_git_prompt_color_dirtystate
        or set -g __fish_git_prompt_color_dirtystate blue
        set -q __fish_git_prompt_char_dirtystate
        or set -g __fish_git_prompt_char_dirtystate '*'
        set -q __fish_git_prompt_char_stagedstate
        or set -g __fish_git_prompt_char_stagedstate '✚'
        set -q __fish_git_prompt_color_cleanstate
        or set -g __fish_git_prompt_color_cleanstate green
        set -q __fish_git_prompt_char_cleanstate
        or set -g __fish_git_prompt_char_cleanstate '✓'
        set -q __fish_git_prompt_color_stagedstate
        or set -g __fish_git_prompt_color_stagedstate yellow
        set -q __fish_git_prompt_color_branch_dirty
        or set -g __fish_git_prompt_color_branch_dirty red
        set -q __fish_git_prompt_color_branch_staged
        or set -g __fish_git_prompt_color_branch_staged yellow
        set -q __fish_git_prompt_color_branch
        or set -g __fish_git_prompt_color_branch green
        set -q __fish_git_prompt_char_stateseparator
        or set -g __fish_git_prompt_char_stateseparator '⚡'
        fish_vcs_prompt '|%s'
        echo
    
        if not test $last_status -eq 0
                set_color $fish_color_error
        end
    
        echo -n '➤ '
        set_color --reset
end
