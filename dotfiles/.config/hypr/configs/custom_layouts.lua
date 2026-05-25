-- SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
-- SPDX-License-Identifier: GPL-3.0-or-later

--   Copyright (C) 2026 Ahum Maitra

--     This program is free software: you can redistribute it and/or modify
--     it under the terms of the GNU General Public License as published by
--     the Free Software Foundation, either version 3 of the License, or
--     (at your option) any later version.

--     This program is distributed in the hope that it will be useful,
--     but WITHOUT ANY WARRANTY; without even the implied warranty of
--     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
--     GNU General Public License for more details.

--     You should have received a copy of the GNU General Public License
--     along with this program.  If not, see <https://www.gnu.org/licenses/>.


local layouts = {
    grid = {
        recalculate = function(ctx)
            local n = #ctx.targets
            if n == 0 then
                return
            end

            local cols = math.ceil(math.sqrt(n))

            for i, target in ipairs(ctx.targets) do
                target:place(ctx:grid_cell(i, cols))
            end
        end,
    }
}

for name, layout in pairs(layouts) do
    hl.layout.register(name, layout)
end